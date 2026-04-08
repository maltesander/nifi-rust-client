// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces)]

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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicyDto {
    /// The action associated with this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "componentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_reference: Option<ComponentReferenceEntity>,
    /// Whether this policy is configurable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The resource for this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The set of user group IDs associated with this access policy.
    #[serde(rename = "userGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<TenantEntity>>,
    /// The set of user IDs associated with this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// The access policies this user group belongs to. This field was incorrectly defined as an AccessPolicyEntity. For compatibility reasons the field will remain of this type, however only the fields that are present in the AccessPolicySummaryEntity will be populated here.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicyEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AccessPolicyDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// When this content was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicySummaryDto {
    /// The action associated with this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "componentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_reference: Option<ComponentReferenceEntity>,
    /// Whether this policy is configurable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The resource for this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// The access policies this user belongs to.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicySummaryEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AccessPolicySummaryDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

/// The details of the action.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDetailsDto {}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDto {
    #[serde(rename = "actionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_details: Option<ActionDetailsDto>,
    #[serde(rename = "componentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_details: Option<ComponentDetailsDto>,
    /// The action id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The operation that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// The id of the source component.
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source component.
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The type of the source component.
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// The timestamp of the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The identity of the user that performed the action.
    #[serde(rename = "userIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,
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
pub struct AffectedComponentDto {
    /// The number of active threads for the referencing component.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The UUID of this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The UUID of the Process Group that this component is in
    #[serde(rename = "processGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The type of this component
    #[serde(rename = "referenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The validation errors for the component.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}

/// The set of all components in the flow that are referencing this Parameter
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponentEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AffectedComponentDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(rename = "processGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group: Option<ProcessGroupNameDto>,
    /// The type of component referenced
    #[serde(rename = "referenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueDto {
    /// A description for this allowable value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A human readable value that is allowed for the property descriptor.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// A value that is allowed for the property descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Allowable values for the property. If empty then the allowed values are not constrained.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueEntity {
    #[serde(rename = "allowableValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowable_value: Option<AllowableValueDto>,
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
}

/// The Asset.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDto {
    /// The digest of the asset, will be null if the asset content is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The identifier of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates if the content of the asset is missing.
    #[serde(rename = "missingContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_content: Option<bool>,
    /// The name of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetEntity {
    pub asset: Option<AssetDto>,
}

/// A list of identifiers of the assets that are referenced by the parameter
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReferenceDto {
    /// The identifier of the referenced asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the referenced asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsEntity {
    /// The asset entities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetEntity>>,
}

/// The FlowFile attributes this processor writes/updates
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDto {
    /// The attribute name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the attribute before the event took place.
    #[serde(rename = "previousValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,
    /// The attribute value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationDto {
    /// Whether the system requires login through an external Identity Provider
    #[serde(rename = "externalLoginRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_required: Option<bool>,
    /// Whether the system is configured to support login operations
    #[serde(rename = "loginSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_supported: Option<bool>,
    /// Location for initiating login processing
    #[serde(rename = "loginUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_uri: Option<String>,
    /// Location for initiating logout processing
    #[serde(rename = "logoutUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationEntity {
    pub authentication_configuration: Option<AuthenticationConfigurationDto>,
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

/// The batch settings for data transmission.
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    /// The compiler used for the build
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler: Option<String>,
    /// The compiler flags used for the build.
    #[serde(rename = "compilerFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler_flags: Option<String>,
    /// The SCM revision id of the source code used for this build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// The target architecture of the built component.
    #[serde(rename = "targetArch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arch: Option<String>,
    /// The timestamp (milliseconds since Epoch) of the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// The version number of the built component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
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
pub struct BulletinBoardPatternParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<serde_json::Value>,
    #[serde(rename = "rawPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_pattern: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinDto {
    /// The category of this bulletin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The group id of the source component.
    #[serde(rename = "groupId")]
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
    #[serde(rename = "nodeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_address: Option<String>,
    /// The id of the source component.
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source component.
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The type of the source component
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// The stack trace associated with the bulletin, if any.
    #[serde(rename = "stackTrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
    /// When this bulletin was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// When this bulletin was generated in ISO format with full date and milliseconds.
    #[serde(rename = "timestampIso")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_iso: Option<String>,
}

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

/// The details of the artifact that bundled this parameter provider.
#[non_exhaustive]
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
#[non_exhaustive]
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
pub struct ClearBulletinsRequestEntity {
    /// The timestamp from which to clear bulletins (inclusive). This field is required.
    #[serde(rename = "fromTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsResultEntity {
    /// The current bulletins for the component after clearing.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// The number of bulletins that were cleared.
    #[serde(rename = "bulletinsCleared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins_cleared: Option<i32>,
    /// The id of the component for which bulletins were cleared.
    #[serde(rename = "componentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientIdParameter {
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
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

/// The details of the source component.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDetailsDto {}

/// The list of differences for each component in the flow that is not the same between the two flows
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDifferenceDto {
    /// The ID of the component
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The name of the component
    #[serde(rename = "componentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// The type of component
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The differences in the component between the two flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differences: Option<Vec<DifferenceDto>>,
    /// The ID of the Process Group that the component belongs to
    #[serde(rename = "processGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHistoryDto {
    /// The component id.
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The history for the properties of the component.
    #[serde(rename = "propertyHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_history: Option<std::collections::HashMap<String, Option<PropertyHistoryDto>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHistoryEntity {
    pub component_history: Option<ComponentHistoryDto>,
}

/// The full specification of the bundle contents
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentManifest {
    /// Public interfaces defined in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<DefinedType>>,
    /// Controller Services provided in this bundle
    #[serde(rename = "controllerServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<ControllerServiceDefinition>>,
    /// Flow Analysis Rules provided in this bundle
    #[serde(rename = "flowAnalysisRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rules: Option<Vec<FlowAnalysisRuleDefinition>>,
    /// Flow Registry Clients provided in this bundle
    #[serde(rename = "flowRegistryClients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_registry_clients: Option<Vec<FlowRegistryClientDefinition>>,
    /// Parameter Providers provided in this bundle
    #[serde(rename = "parameterProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers: Option<Vec<ParameterProviderDefinition>>,
    /// Processors provided in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorDefinition>>,
    /// Reporting Tasks provided in this bundle
    #[serde(rename = "reportingTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<ReportingTaskDefinition>>,
}

#[non_exhaustive]
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
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// Component this policy references if applicable.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentReferenceEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ComponentReferenceDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRestrictionPermissionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(rename = "requiredPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permission: Option<RequiredPermissionDto>,
}

/// The parameters that matched the search.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSearchResultDto {
    /// The group id of the component that matched the search.
    #[serde(rename = "groupId")]
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
    #[serde(rename = "parentGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group: Option<SearchResultGroupDto>,
    #[serde(rename = "versionedGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_group: Option<SearchResultGroupDto>,
}

/// The component state.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStateDto {
    #[serde(rename = "clusterState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<StateMapDto>,
    /// The component identifier.
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// Whether dropping state by key is supported for this component. Defaults to false when not specified by the component.
    #[serde(rename = "dropStateKeySupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_state_key_supported: Option<bool>,
    #[serde(rename = "localState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_state: Option<StateMapDto>,
    /// Description of the state this component persists.
    #[serde(rename = "stateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStateEntity {
    pub component_state: Option<ComponentStateDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultDto {
    /// The number of active threads for the referencing component.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// Whether or not the component is currently valid
    #[serde(rename = "currentlyValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_valid: Option<bool>,
    /// The UUID of this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The UUID of the Process Group that this component is in
    #[serde(rename = "processGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The type of this component
    #[serde(rename = "referenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    /// The validation errors that will apply to the component if the Parameter Context is changed
    #[serde(rename = "resultantValidationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resultant_validation_errors: Option<Vec<String>>,
    /// Whether or not the component will be valid if the Parameter Context is changed
    #[serde(rename = "resultsValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_valid: Option<bool>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The validation errors for the component.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}

/// A List of ComponentValidationResultEntity, one for each component that is validated
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ComponentValidationResultDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultsEntity {
    /// A List of ComponentValidationResultEntity, one for each component that is validated
    #[serde(rename = "validationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_results: Option<Vec<ComponentValidationResultEntity>>,
}

/// The Results of the verification
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigVerificationResultDto {
    /// An explanation of why the step was or was not successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// The outcome of the verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    /// The name of the verification step
    #[serde(rename = "verificationStepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_step_name: Option<String>,
}

/// The configuration analysis
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationAnalysisDto {
    /// The ID of the component
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The configured properties for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The attributes that are referenced by the properties, mapped to recently used values
    #[serde(rename = "referencedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether or not the component supports verification
    #[serde(rename = "supportsVerification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_verification: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationAnalysisEntity {
    pub configuration_analysis: Option<ConfigurationAnalysisDto>,
}

/// The destination of the connection.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectableComponent {
    /// The comments for the connectable component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The id of the group that the connectable component resides in
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the connectable component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The name of the connectable component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of component the connectable is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// The destination of the connection.
#[non_exhaustive]
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
    #[serde(rename = "groupId")]
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
    pub r#type: String,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// The connections in this flow snippet.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDto {
    /// The relationships that the source of the connection currently supports.
    #[serde(rename = "availableRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_relationships: Option<Vec<String>>,
    /// The object data size threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    #[serde(rename = "backPressureDataSizeThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_data_size_threshold: Option<String>,
    /// The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    #[serde(rename = "backPressureObjectThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_object_threshold: Option<i64>,
    /// The bend points on the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bends: Option<Vec<PositionDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ConnectableDto>,
    /// The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it.
    #[serde(rename = "flowFileExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_expiration: Option<String>,
    /// The z index of the connection.
    #[serde(rename = "getzIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getz_index: Option<i64>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The index of the bend point where to place the connection label.
    #[serde(rename = "labelIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_index: Option<i32>,
    /// Whether or not data should be compressed when being transferred between nodes in the cluster. Possible returned values: DO_NOT_COMPRESS, COMPRESS_ATTRIBUTES_ONLY, COMPRESS_ATTRIBUTES_AND_CONTENT. See LoadBalanceCompression.class for more details.
    #[serde(rename = "loadBalanceCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_compression: Option<String>,
    /// The FlowFile Attribute to use for determining which node a FlowFile will go to if the Load Balancing Strategy is set to PARTITION_BY_ATTRIBUTE
    #[serde(rename = "loadBalancePartitionAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_partition_attribute: Option<String>,
    /// The current status of the Connection's Load Balancing Activities. Status can indicate that Load Balancing is not configured for the connection, that Load Balancing is configured but inactive (not currently transferring data to another node), or that Load Balancing is configured and actively transferring data to another node. Possible returned values: LOAD_BALANCE_NOT_CONFIGURED, LOAD_BALANCE_INACTIVE, LOAD_BALANCE_ACTIVE. See LoadBalanceStatus.class for more details.
    #[serde(rename = "loadBalanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_status: Option<String>,
    /// How to load balance the data in this Connection across the nodes in the cluster. Possible returned values: DO_NOT_LOAD_BALANCE, PARTITION_BY_ATTRIBUTE, ROUND_ROBIN, SINGLE_NODE. See LoadBalanceStrategy.class for more details.
    #[serde(rename = "loadBalanceStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_strategy: Option<String>,
    /// The name of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The comparators used to prioritize the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritizers: Option<Vec<String>>,
    /// The relationships from the source of the connection that are configured to be retried.
    #[serde(rename = "retriedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_relationships: Option<Vec<String>>,
    /// The selected relationship that comprise the connection.
    #[serde(rename = "selectedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_relationships: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ConnectableDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// The connections in this flow.
#[non_exhaustive]
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
    #[serde(rename = "destinationGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_group_id: Option<String>,
    /// The identifier of the destination of this connection.
    #[serde(rename = "destinationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// The type of component the destination connectable is.
    #[serde(rename = "destinationType")]
    pub destination_type: String,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The z index of the connection.
    #[serde(rename = "getzIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getz_index: Option<i64>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The index of the bend point where to place the connection label.
    #[serde(rename = "labelIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The identifier of the group of the source of this connection.
    #[serde(rename = "sourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_group_id: Option<String>,
    /// The identifier of the source of this connection.
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The type of component the source connectable is.
    #[serde(rename = "sourceType")]
    pub source_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConnectionStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ConnectionStatisticsSnapshotDto>,
    /// The ID of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of status snapshots for each node
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeConnectionStatisticsSnapshotDto>>,
    /// The timestamp of when the stats were last refreshed
    #[serde(rename = "statsLastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
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

/// The connection status snapshot from the node.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsSnapshotDto {
    /// The id of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The predicted total number of bytes in the queue at the next configured interval.
    #[serde(rename = "predictedBytesAtNextInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_bytes_at_next_interval: Option<i64>,
    /// The predicted number of queued objects at the next configured interval.
    #[serde(rename = "predictedCountAtNextInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_count_at_next_interval: Option<i32>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the total number of bytes in the queue.
    #[serde(rename = "predictedMillisUntilBytesBackpressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the queued count.
    #[serde(rename = "predictedMillisUntilCountBackpressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_count_backpressure: Option<i64>,
    /// The predicted percentage of bytes in the queue against current threshold at the next configured interval.
    #[serde(rename = "predictedPercentBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_bytes: Option<i32>,
    /// The predicted percentage of queued objects at the next configured interval.
    #[serde(rename = "predictedPercentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_count: Option<i32>,
    /// The prediction interval in seconds
    #[serde(rename = "predictionIntervalMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_millis: Option<i64>,
}

/// The status of the connection.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ConnectionStatusSnapshotDto>,
    /// The ID of the destination component
    #[serde(rename = "destinationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// The name of the destination component
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    /// The ID of the Process Group that the connection belongs to
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The ID of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of status snapshots for each node
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeConnectionStatusSnapshotDto>>,
    /// The ID of the source component
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source component
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The timestamp of when the stats were last refreshed
    #[serde(rename = "statsLastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
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

/// Predictions, if available, for this connection (null if not available)
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusPredictionsSnapshotDto {
    /// The predicted total number of bytes in the queue at the next configured interval.
    #[serde(rename = "predictedBytesAtNextInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_bytes_at_next_interval: Option<i64>,
    /// The predicted number of queued objects at the next configured interval.
    #[serde(rename = "predictedCountAtNextInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_count_at_next_interval: Option<i32>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the total number of bytes in the queue.
    #[serde(rename = "predictedMillisUntilBytesBackpressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the queued count.
    #[serde(rename = "predictedMillisUntilCountBackpressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_count_backpressure: Option<i64>,
    /// Predicted connection percent use regarding queued flow files size and backpressure threshold if configured.
    #[serde(rename = "predictedPercentBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_bytes: Option<i32>,
    /// Predicted connection percent use regarding queued flow files count and backpressure threshold if configured.
    #[serde(rename = "predictedPercentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_count: Option<i32>,
    /// The configured interval (in seconds) for predicting connection queue count and size (and percent usage).
    #[serde(rename = "predictionIntervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_seconds: Option<i32>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusSnapshotDto {
    /// The size of the FlowFiles that have come into the connection in the last 5 minutes.
    #[serde(rename = "bytesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The number of bytes that have left the connection in the last 5 minutes.
    #[serde(rename = "bytesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The size of the FlowFiles that are currently queued in the connection.
    #[serde(rename = "bytesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_queued: Option<i64>,
    /// The id of the destination of the connection.
    #[serde(rename = "destinationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// The name of the destination of the connection.
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    /// The availability of FlowFiles in this connection
    #[serde(rename = "flowFileAvailability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_availability: Option<String>,
    /// The number of FlowFiles that have come into the connection in the last 5 minutes.
    #[serde(rename = "flowFilesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles that have left the connection in the last 5 minutes.
    #[serde(rename = "flowFilesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The number of FlowFiles that are currently queued in the connection.
    #[serde(rename = "flowFilesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_queued: Option<i32>,
    /// The id of the process group the connection belongs to.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input count/size for the connection in the last 5 minutes, pretty printed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The load balance status of the connection
    #[serde(rename = "loadBalanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_status: Option<String>,
    /// The name of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The output count/size for the connection in the last 5 minutes, pretty printed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Connection percent use regarding queued flow files size and backpressure threshold if configured.
    #[serde(rename = "percentUseBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_use_bytes: Option<i32>,
    /// Connection percent use regarding queued flow files count and backpressure threshold if configured.
    #[serde(rename = "percentUseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_use_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictions: Option<ConnectionStatusPredictionsSnapshotDto>,
    /// The total count and size of queued flowfiles formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    /// The number of flowfiles that are queued, pretty printed.
    #[serde(rename = "queuedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_count: Option<String>,
    /// The total size of flowfiles that are queued formatted.
    #[serde(rename = "queuedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_size: Option<String>,
    /// The id of the source of the connection.
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source of the connection.
    #[serde(rename = "sourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}

/// The status of all connections in the process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "connectionStatusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status_snapshot: Option<ConnectionStatusSnapshotDto>,
    /// The id of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ConnectionEntity>>,
}

/// The Content Viewers.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerDto {
    /// The display name of the Content Viewer.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The mime types this Content Viewer supports.
    #[serde(rename = "supportedMimeTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_mime_types: Option<Vec<SupportedMimeTypesDto>>,
    /// The uri of the Content Viewer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
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

/// The controller configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerConfigurationDto {
    /// The maximum number of timer driven threads the NiFi has available.
    #[serde(rename = "maxTimerDrivenThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timer_driven_thread_count: Option<i32>,
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
pub struct ControllerDto {
    /// The number of active remote ports contained in the NiFi.
    #[serde(rename = "activeRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The comments for the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of disabled components in the NiFi.
    #[serde(rename = "disabledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The id of the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of inactive remote ports contained in the NiFi.
    #[serde(rename = "inactiveRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports contained in the NiFi.
    #[serde(rename = "inputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The input ports available to send data to for the NiFi.
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortDto>>,
    /// If clustered, the id of the Cluster Manager, otherwise the id of the NiFi.
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// The number of invalid components in the NiFi.
    #[serde(rename = "invalidCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The name of the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of output ports in the NiFi.
    #[serde(rename = "outputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    /// The output ports available to received data from the NiFi.
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortDto>>,
    /// The HTTP(S) Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    #[serde(rename = "remoteSiteHttpListeningPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_site_http_listening_port: Option<i32>,
    /// The Socket Port on which this instance is listening for Remote Transfers of Flow Files. If this instance is not configured to receive Flow Files from remote instances, this will be null.
    #[serde(rename = "remoteSiteListeningPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_site_listening_port: Option<i32>,
    /// The number of running components in the NiFi.
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// Indicates whether or not Site-to-Site communications with this instance is secure (2-way authentication).
    #[serde(rename = "siteToSiteSecure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_to_site_secure: Option<bool>,
    /// The number of stopped components in the NiFi.
    #[serde(rename = "stoppedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerEntity {
    pub controller: Option<ControllerDto>,
}

/// Lists the APIs this Controller Service implements.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceAPI {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The fully qualified name of the service interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Lists the APIs this Controller Service implements.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceApiDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The fully qualified name of the service interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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

/// The controller services in this flow snippet.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceDto {
    /// The annotation for the controller service. This is how the custom UI relays configuration to the controller service.
    #[serde(rename = "annotationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The level at which the controller service will report bulletins.
    #[serde(rename = "bulletinLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The comments for the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Lists the APIs this Controller Service implements.
    #[serde(rename = "controllerServiceApis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    /// The URL for the controller services custom configuration UI if applicable.
    #[serde(rename = "customUiUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// Whether the ontroller service has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The descriptors for the controller service properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    #[serde(rename = "extensionMissing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the controller service has multiple versions available.
    #[serde(rename = "multipleVersionsAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the controller service persists state.
    #[serde(rename = "persistsState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The properties of the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// All components referencing this controller service.
    #[serde(rename = "referencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    /// Whether the controller service requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// Set of sensitive dynamic property names
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Whether the controller service supports sensitive dynamic properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The validation errors from the controller service.
    /// These validation errors represent the problems with the controller service that must be resolved before it can be enabled.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the ControllerService is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the ControllerService is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ControllerServiceDto>,
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
    /// The id of parent process group of this ControllerService.
    #[serde(rename = "parentGroupId")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentDto {
    /// The number of active threads for the referencing component.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The descriptors for the component properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// The group id for the component referencing a controller service. If this component is another controller service or a reporting task, this field is blank.
    #[serde(rename = "groupId")]
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
    #[serde(rename = "referenceCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_cycle: Option<bool>,
    /// The type of reference this is.
    #[serde(rename = "referenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    /// If the referencing component represents a controller service, these are the components that reference it.
    #[serde(rename = "referencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The type of the component referencing a controller service in simple Java class name format without package name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The validation errors for the component.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}

/// All components referencing this controller service.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ControllerServiceReferencingComponentDto>,
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
pub struct ControllerServiceReferencingComponentsEntity {
    #[serde(rename = "controllerServiceReferencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_referencing_components:
        Option<Vec<ControllerServiceReferencingComponentEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the ControllerService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Indicates whether or not responses should only include fields necessary for rendering the NiFi User Interface.
    /// As such, when this value is set to true, some fields may be returned as null values, and the selected fields may change at any time without notice.
    /// As a result, this value should not be set to true by any client other than the UI.
    #[serde(rename = "uiOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_only: Option<bool>,
}

/// The status for this ControllerService.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceStatusDto {
    /// The number of active threads for the component.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The run status of this ControllerService
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
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
pub struct CopyRequestEntity {
    /// The ids of the connections to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<String>>,
    /// The ids of the funnels to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<String>>,
    /// The ids of the input ports to be copied.
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<String>>,
    /// The ids of the labels to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// The ids of the output ports to be copied.
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<String>>,
    /// The ids of the process groups to be copied.
    #[serde(rename = "processGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<String>>,
    /// The ids of the processors to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<String>>,
    /// The ids of the remote process groups to be copied.
    #[serde(rename = "remoteProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<String>>,
}

/// The response from copying.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyResponseEntity {
    /// The connections being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<VersionedConnection>>,
    /// The external controller service references.
    #[serde(rename = "externalControllerServiceReferences")]
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
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<VersionedPort>>,
    /// The labels being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<VersionedLabel>>,
    /// The output ports being copied.
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<VersionedPort>>,
    /// The referenced parameter contexts.
    #[serde(rename = "parameterContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    /// The referenced parameter providers.
    #[serde(rename = "parameterProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    /// The process groups being copied.
    #[serde(rename = "processGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<VersionedProcessGroup>>,
    /// The processors being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<VersionedProcessor>>,
    /// The remote process groups being copied.
    #[serde(rename = "remoteProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopySnippetRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The x coordinate of the origin of the bounding box where the new components will be placed.
    #[serde(rename = "originX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_x: Option<f64>,
    /// The y coordinate of the origin of the bounding box where the new components will be placed.
    #[serde(rename = "originY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_y: Option<f64>,
    /// The identifier of the snippet.
    #[serde(rename = "snippetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterDto {
    /// The context of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// The id of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the counter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// The value count.
    #[serde(rename = "valueCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterEntity {
    pub counter: Option<CounterDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<CountersSnapshotDto>,
    /// A Counters snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeCountersSnapshotDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersEntity {
    pub counters: Option<CountersDto>,
}

/// The counters from the node.
#[non_exhaustive]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActiveRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The Process Group ID that this active request will update
    #[serde(rename = "processGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
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
pub struct DateTimeParameter {
    #[serde(rename = "dateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
}

/// Indicates that this property is for selecting a controller service of the specified type
#[non_exhaustive]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum DiagnosticLevel {
    #[serde(rename = "BASIC")]
    Basic,
    #[serde(rename = "VERBOSE")]
    Verbose,
}

impl std::fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticLevel::Basic => write!(f, "BASIC"),
            DiagnosticLevel::Verbose => write!(f, "VERBOSE"),
        }
    }
}

/// The differences in the component between the two flows
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferenceDto {
    /// Description of the difference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference: Option<String>,
    /// The type of difference
    #[serde(rename = "differenceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference_type: Option<String>,
}

#[non_exhaustive]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentedTypeDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// If this type represents a ControllerService, this lists the APIs it implements.
    #[serde(rename = "controllerServiceApis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    /// The description of why the usage of this component is restricted.
    #[serde(rename = "deprecationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An optional collection of explicit restrictions. If specified, these explicit restrictions will be enfored.
    #[serde(rename = "explicitRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<ExplicitRestrictionDto>>,
    /// Whether this type is restricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// The tags associated with this type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully qualified name of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The optional description of why the usage of this component is restricted.
    #[serde(rename = "usageRestriction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_restriction: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRequestDto {
    /// The count and size of flow files currently queued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    /// The number of flow files currently queued.
    #[serde(rename = "currentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_count: Option<i32>,
    /// The size of flow files currently queued in bytes.
    #[serde(rename = "currentSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_size: Option<i64>,
    /// The count and size of flow files that have been dropped thus far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped: Option<String>,
    /// The number of flow files that have been dropped thus far.
    #[serde(rename = "droppedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped_count: Option<i32>,
    /// The size of flow files that have been dropped thus far in bytes.
    #[serde(rename = "droppedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped_size: Option<i64>,
    /// The reason, if any, that this drop request failed.
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Whether the query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The id for this drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The last time this drop request was updated.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The count and size of flow files to be dropped as a result of this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    /// The number of flow files to be dropped as a result of this request.
    #[serde(rename = "originalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_count: Option<i32>,
    /// The size of flow files to be dropped as a result of this request in bytes.
    #[serde(rename = "originalSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_size: Option<i64>,
    /// The current percent complete.
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The current state of the drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp when the query was submitted.
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for future requests to this drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRequestEntity {
    pub drop_request: Option<DropRequestDto>,
}

/// Describes the dynamic properties supported by this component
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicProperty {
    /// The description of the dynamic property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The scope of the expression language support
    #[serde(rename = "expressionLanguageScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope: Option<String>,
    /// The description of the dynamic property name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the dynamic property value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// If the processor supports dynamic relationships, this describes the dynamic relationship
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplicitRestrictionDto {
    /// The description of why the usage of this component is restricted for this required permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(rename = "requiredPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permission: Option<RequiredPermissionDto>,
}

#[non_exhaustive]
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
    #[serde(rename = "enforcementPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_policy: Option<String>,
    /// Whether the underlying extension is missing.
    #[serde(rename = "extensionMissing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the flow analysis rule has multiple versions available.
    #[serde(rename = "multipleVersionsAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the flow analysis rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the flow analysis rule persists state.
    #[serde(rename = "persistsState")]
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
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the flow analysis rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Whether the flow analysis rule supports sensitive dynamic properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The fully qualified type of the flow analysis rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the flow analysis rule. These validation errors represent the problems with the flow analysis rule that must be resolved before it can be scheduled to run.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Flow Analysis Rule is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Flow Analysis Rule is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
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

/// The status for this FlowAnalysisRule.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleStatusDto {
    /// The number of active threads for the component.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The run status of this FlowAnalysisRule
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleTypesEntity {
    #[serde(rename = "flowAnalysisRuleTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rule_types: Option<Vec<DocumentedTypeDto>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleViolationDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "enforcementPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_policy: Option<String>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "issueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "subjectComponentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_component_type: Option<String>,
    #[serde(rename = "subjectDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_display_name: Option<String>,
    #[serde(rename = "subjectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "subjectPermissionDto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_permission_dto: Option<PermissionsDto>,
    #[serde(rename = "violationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_message: Option<String>,
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

/// This breadcrumb.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBreadcrumbDto {
    /// The id of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "versionControlInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowComparisonEntity {
    /// The list of differences for each component in the flow that is not the same between the two flows
    #[serde(rename = "componentDifferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_differences: Option<Vec<ComponentDifferenceDto>>,
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

/// Flow containing the components that were created as part of this paste action.
#[non_exhaustive]
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
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortEntity>>,
    /// The labels in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelEntity>>,
    /// The output ports in this flow.
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortEntity>>,
    /// The process groups in this flow.
    #[serde(rename = "processGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
    /// The processors in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorEntity>>,
    /// The remote process groups in this flow.
    #[serde(rename = "remoteProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowEntity {
    pub flow: Option<FlowDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileDto {
    /// The FlowFile attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// The label for the node where this FlowFile resides.
    #[serde(rename = "clusterNodeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The id of the node where this FlowFile resides.
    #[serde(rename = "clusterNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The container in which the content claim lives.
    #[serde(rename = "contentClaimContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_container: Option<String>,
    /// The file size of the content claim formatted.
    #[serde(rename = "contentClaimFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_file_size: Option<String>,
    /// The file size of the content claim in bytes.
    #[serde(rename = "contentClaimFileSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the content claim.
    #[serde(rename = "contentClaimIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_identifier: Option<String>,
    /// The offset into the content claim where the flowfile's content begins.
    #[serde(rename = "contentClaimOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_offset: Option<i64>,
    /// The section in which the content claim lives.
    #[serde(rename = "contentClaimSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_claim_section: Option<String>,
    /// The FlowFile filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Duration since the FlowFile's greatest ancestor entered the flow.
    #[serde(rename = "lineageDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// The FlowFile mime type.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// If the FlowFile is penalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalized: Option<bool>,
    /// How long in milliseconds until the FlowFile penalty expires.
    #[serde(rename = "penaltyExpiresIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_expires_in: Option<i64>,
    /// The FlowFile's position in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// How long this FlowFile has been enqueued.
    #[serde(rename = "queuedDuration")]
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
pub struct FlowFileEntity {
    pub flow_file: Option<FlowFileDto>,
}

/// The FlowFile summaries. The summaries will be populated once the request has completed.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileSummaryDto {
    /// The label for the node where this FlowFile resides.
    #[serde(rename = "clusterNodeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The id of the node where this FlowFile resides.
    #[serde(rename = "clusterNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The FlowFile filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Duration since the FlowFile's greatest ancestor entered the flow.
    #[serde(rename = "lineageDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// The FlowFile mime type.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// If the FlowFile is penalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalized: Option<bool>,
    /// How long in milliseconds until the FlowFile penalty expires.
    #[serde(rename = "penaltyExpiresIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_expires_in: Option<i64>,
    /// The FlowFile's position in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// How long this FlowFile has been enqueued.
    #[serde(rename = "queuedDuration")]
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
            FlowMetricsReportingStrategy::AllProcessGroups => write!(f, "ALL_PROCESS_GROUPS"),
        }
    }
}

#[non_exhaustive]
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
    pub branch: Option<FlowRegistryBranchDto>,
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
pub struct FlowRegistryBucket {
    #[serde(rename = "createdTimestamp")]
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

#[non_exhaustive]
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

#[non_exhaustive]
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
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientDto {
    /// The annotation data for the registry client. This is how the custom UI relays configuration to the registry client.
    #[serde(rename = "annotationData")]
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
    #[serde(rename = "extensionMissing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The registry identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the flow registry client has multiple versions available.
    #[serde(rename = "multipleVersionsAvailable")]
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
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// Whether the registry client supports branching.
    #[serde(rename = "supportsBranching")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_branching: Option<bool>,
    /// Whether the registry client supports sensitive dynamic properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the registry client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the registry client. These validation errors represent the problems with the registry client that must be resolved before it can be used for interacting with the flow registry.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Registry Client is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Registry Client is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
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
pub struct FlowRegistryClientsEntity {
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registries: Option<Vec<FlowRegistryClientEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryPermissions {
    #[serde(rename = "canDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "canWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_write: Option<bool>,
}

/// The contents of this process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSnippetDto {
    /// The connections in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ConnectionDto>>,
    /// The controller services in this flow snippet.
    #[serde(rename = "controllerServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<ControllerServiceDto>>,
    /// The funnels in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<FunnelDto>>,
    /// The input ports in this flow snippet.
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortDto>>,
    /// The labels in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelDto>>,
    /// The output ports in this flow snippet.
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortDto>>,
    /// The process groups in this flow snippet.
    #[serde(rename = "processGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<ProcessGroupDto>>,
    /// The processors in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorDto>>,
    /// The remote process groups in this flow snippet.
    #[serde(rename = "remoteProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupDto>>,
}

/// The funnels in this flow snippet.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelDto {
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// The funnels in this flow.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<FunnelDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<FunnelEntity>>,
}

/// The garbage collection details.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageCollectionDto {
    /// The number of times garbage collection has run.
    #[serde(rename = "collectionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_count: Option<i64>,
    /// The total number of milliseconds spent garbage collecting.
    #[serde(rename = "collectionMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_millis: Option<i64>,
    /// The total amount of time spent garbage collecting.
    #[serde(rename = "collectionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_time: Option<String>,
    /// The name of the garbage collector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryDto {
    /// The actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ActionEntity>>,
    /// The timestamp when the report was generated.
    #[serde(rename = "lastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed: Option<String>,
    /// The number of number of actions that matched the search criteria..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntity {
    pub history: Option<HistoryDto>,
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
pub struct InputPortsEntity {
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegerParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer: Option<i32>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultDto {
    /// The attribute name of the metrics bean's attribute.
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// The attribute value of the the metrics bean's attribute
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<serde_json::Value>,
    /// The bean name of the metrics bean.
    #[serde(rename = "beanName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bean_name: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultsEntity {
    #[serde(rename = "jmxMetricsResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmx_metrics_results: Option<Vec<JmxMetricsResultDto>>,
}

/// The labels in this flow snippet.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelDto {
    /// The z index of the label.
    #[serde(rename = "getzIndex")]
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
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The styles for this label (font-size : 12px, background-color : #eee, etc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
    /// The width of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}

/// The labels in this flow.
#[non_exhaustive]
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
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The z index of the label.
    #[serde(rename = "getzIndex")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestProvenanceEventsDto {
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[serde(rename = "provenanceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestProvenanceEventsEntity {
    pub latest_provenance_events: Option<LatestProvenanceEventsDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageDto {
    /// When the lineage query will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Whether the lineage query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The id of this lineage query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The percent complete for the lineage query.
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<LineageRequestDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<LineageResultsDto>,
    /// When the lineage query was submitted.
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for this lineage query for later retrieval and deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageEntity {
    pub lineage: Option<LineageDto>,
}

/// The initial lineage result.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageRequestDto {
    /// The id of the node where this lineage originated if clustered.
    #[serde(rename = "clusterNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The event id that was used to generate this lineage, if applicable.
    /// The event id is allowed for any type of lineageRequestType.
    /// If the lineageRequestType is FLOWFILE and the flowfile uuid is also included in the request, the event id will be ignored.
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    /// The type of lineage request. PARENTS will return the lineage for the flowfiles that are parents of the specified event. CHILDREN will return the lineage for the flowfiles that are children of the specified event. FLOWFILE will return the lineage for the specified flowfile.
    #[serde(rename = "lineageRequestType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_request_type: Option<String>,
    /// The flowfile uuid that was used to generate the lineage. The flowfile uuid is only allowed when the lineageRequestType is FLOWFILE and will take precedence over event id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// The results of the lineage query.
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortDto {
    /// Supported application protocols, if applicable
    #[serde(rename = "applicationProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocols: Option<Vec<String>>,
    /// The class type of the component providing the listen port
    #[serde(rename = "componentClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_class: Option<String>,
    /// The id of the component providing the listen port
    #[serde(rename = "componentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The name of the component providing the listen port
    #[serde(rename = "componentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// The type of component providing the listen port (e.g., Processor, ControllerService)
    #[serde(rename = "componentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The id of the process group containing the component providing the listen port, if applicable
    #[serde(rename = "parentGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// The name of the process group containing the component providing the listen port, if applicable
    #[serde(rename = "parentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_name: Option<String>,
    /// The name of the the listen port. Useful context for components that provide multiple ports.
    #[serde(rename = "portName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
    /// The ingress port number
    #[serde(rename = "portNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    /// The ingress transport protocol (TCP or UDP)
    #[serde(rename = "transportProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
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
pub struct ListingRequestDto {
    /// Whether the destination of the connection is running
    #[serde(rename = "destinationRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_running: Option<bool>,
    /// The reason, if any, that this listing request failed.
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Whether the query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The FlowFile summaries. The summaries will be populated once the request has completed.
    #[serde(rename = "flowFileSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_summaries: Option<Vec<FlowFileSummaryDto>>,
    /// The id for this listing request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The last time this listing request was updated.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The maximum number of FlowFileSummary objects to return
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The current percent complete.
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "queueSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<QueueSizeDto>,
    /// Whether the source of the connection is running
    #[serde(rename = "sourceRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_running: Option<bool>,
    /// The current state of the listing request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp when the query was submitted.
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for future requests to this listing request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingRequestEntity {
    pub listing_request: Option<ListingRequestDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LongParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<i64>,
}

/// A list of use cases that have been documented that involve this Processor in conjunction with other Processors
#[non_exhaustive]
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
#[non_exhaustive]
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

/// A list of status snapshots for each node
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatisticsSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "statisticsSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_snapshot: Option<ConnectionStatisticsSnapshotDto>,
}

/// A list of status snapshots for each node
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<ConnectionStatusSnapshotDto>,
}

/// A Counters snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCountersSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<CountersSnapshotDto>,
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

/// The node's events.
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePortStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<PortStatusSnapshotDto>,
}

/// The status reported by each node in the cluster. If the NiFi instance is a standalone instance, rather than a clustered instance, this value may be null.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessGroupStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}

/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessorStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<ProcessorStatusSnapshotDto>,
}

/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRemoteProcessGroupStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}

/// The node-wise results
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeReplayLastEventSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<ReplayLastEventSnapshotDto>,
}

#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatusSnapshotsDto {
    /// The node's host/ip address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The port the node is listening for API requests.
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The id of the node.
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// A list of StatusSnapshotDTO objects that provide the actual metric values for the component for this node.
    #[serde(rename = "statusSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshots: Option<Vec<StatusSnapshotDto>>,
}

/// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemDiagnosticsSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<SystemDiagnosticsSnapshotDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPortsEntity {
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortEntity>>,
}

/// The Parameter Context that is being operated on. This may not be populated until the request has successfully completed.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextDto {
    /// The Process Groups that are bound to this Parameter Context
    #[serde(rename = "boundProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_process_groups: Option<Vec<ProcessGroupEntity>>,
    /// The Description of the Parameter Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID the Parameter Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of references of Parameter Contexts from which this one inherits parameters
    #[serde(rename = "inheritedParameterContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_parameter_contexts: Option<Vec<ParameterContextReferenceEntity>>,
    /// The Name of the Parameter Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parameterProviderConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_configuration: Option<ParameterProviderConfigurationEntity>,
    /// The Parameters for the Parameter Context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterContextDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ParameterContextHandlingStrategy {
    #[serde(rename = "KEEP_EXISTING")]
    KeepExisting,
    #[serde(rename = "REPLACE")]
    Replace,
}

impl std::fmt::Display for ParameterContextHandlingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterContextHandlingStrategy::KeepExisting => write!(f, "KEEP_EXISTING"),
            ParameterContextHandlingStrategy::Replace => write!(f, "REPLACE"),
        }
    }
}

#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateEntity {
    #[serde(rename = "parameterContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextDto>,
    #[serde(rename = "parameterContextRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_revision: Option<RevisionDto>,
    /// The components that are referenced by the update.
    #[serde(rename = "referencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
}

/// The Update Request
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateRequestDto {
    /// Whether or not the request is completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The reason for the request failing, or null if the request has not failed
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "parameterContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The components that are referenced by the update.
    #[serde(rename = "referencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// The ID of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// A description of the current state of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each
    #[serde(rename = "updateSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<ParameterContextUpdateStepDto>>,
    /// The URI for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateRequestEntity {
    #[serde(rename = "parameterContextRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ParameterContextUpdateRequestDto>,
}

/// The steps that are required in order to complete the request, along with the status of each
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateStepDto {
    /// Whether or not this step has completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

/// The Update Request
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestDto {
    /// Whether or not the request is completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    #[serde(rename = "componentValidationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_validation_results: Option<ComponentValidationResultsEntity>,
    /// The reason for the request failing, or null if the request has not failed
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "parameterContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The ID of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// A description of the current state of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each
    #[serde(rename = "updateSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<ParameterContextValidationStepDto>>,
    /// The URI for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ParameterContextValidationRequestDto>,
}

/// The steps that are required in order to complete the request, along with the status of each
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationStepDto {
    /// Whether or not this step has completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
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

/// The parameter information
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDto {
    /// The description of the Parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not the Parameter is inherited from another context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /// The name of the Parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parameterContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// Whether or not the Parameter is provided by a ParameterProvider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided: Option<bool>,
    /// A list of identifiers of the assets that are referenced by the parameter
    #[serde(rename = "referencedAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_assets: Option<Vec<AssetReferenceDto>>,
    /// The set of all components in the flow that are referencing this Parameter
    #[serde(rename = "referencingComponents")]
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
    #[serde(rename = "valueRemoved")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_removed: Option<bool>,
}

/// The name of the Parameter
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    /// Indicates whether the user can write a given resource.
    #[serde(rename = "canWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<ParameterDto>,
}

/// Configuration for any fetched parameter groups.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterGroupConfigurationEntity {
    /// The name of the external parameter group to which the provided parameter names apply.
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// The name of the ParameterContext that receives the parameters in this group
    #[serde(rename = "parameterContextName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_name: Option<String>,
    /// All fetched parameter names that should be applied.
    #[serde(rename = "parameterSensitivities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_sensitivities: Option<std::collections::HashMap<String, Option<String>>>,
    /// True if this group should be synchronized to a ParameterContext, including creating one if it does not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronized: Option<bool>,
}

/// The Apply Parameters Request
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersRequestDto {
    /// Whether or not the request is completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The reason for the request failing, or null if the request has not failed
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The Parameter Contexts updated by this Parameter Provider. This may not be populated until the request has successfully completed.
    #[serde(rename = "parameterContextUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_updates: Option<Vec<ParameterContextUpdateEntity>>,
    #[serde(rename = "parameterProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider: Option<ParameterProviderDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The components that are referenced by the update.
    #[serde(rename = "referencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// The ID of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// A description of the current state of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each
    #[serde(rename = "updateSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<ParameterProviderApplyParametersUpdateStepDto>>,
    /// The URI for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersRequestEntity {
    pub request: Option<ParameterProviderApplyParametersRequestDto>,
}

/// The steps that are required in order to complete the request, along with the status of each
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersUpdateStepDto {
    /// Whether or not this step has completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationDto {
    /// The Parameter Group name that maps to the Parameter Context
    #[serde(rename = "parameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// The ID of the Parameter Provider
    #[serde(rename = "parameterProviderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_id: Option<String>,
    /// The name of the Parameter Provider
    #[serde(rename = "parameterProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_name: Option<String>,
    /// True if the Parameter Context should receive the parameters from the mapped Parameter Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronized: Option<bool>,
}

/// Optional configuration for a Parameter Provider
#[non_exhaustive]
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
pub struct ParameterProviderDto {
    /// The set of all components in the flow that are referencing Parameters provided by this provider
    #[serde(rename = "affectedComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_components: Option<Vec<AffectedComponentEntity>>,
    /// The annotation data for the parameter provider. This is how the custom UI relays configuration to the parameter provider.
    #[serde(rename = "annotationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The comments of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The URL for the custom configuration UI for the parameter provider.
    #[serde(rename = "customUiUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// Whether the parameter provider has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The descriptors for the parameter providers properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    #[serde(rename = "extensionMissing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the parameter provider has multiple versions available.
    #[serde(rename = "multipleVersionsAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Configuration for any fetched parameter groups.
    #[serde(rename = "parameterGroupConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    /// The status of all provided parameters for this parameter provider
    #[serde(rename = "parameterStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_status: Option<Vec<ParameterStatusDto>>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the parameter provider persists state.
    #[serde(rename = "persistsState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The properties of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The Parameter Contexts that reference this Parameter Provider
    #[serde(rename = "referencingParameterContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_parameter_contexts: Option<Vec<ParameterProviderReferencingComponentEntity>>,
    /// Whether the parameter provider requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// The fully qualified type of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the parameter provider. These validation errors represent the problems with the parameter provider that must be resolved before it can be scheduled to run.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Parameter Provider is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Parameter Provider is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterProviderDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterApplicationEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Configuration for the fetched Parameter Groups
    #[serde(rename = "parameterGroupConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterFetchEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}

#[non_exhaustive]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterProviderReferencingComponentDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentsEntity {
    #[serde(rename = "parameterProviderReferencingComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_referencing_components:
        Option<Vec<ParameterProviderReferencingComponentEntity>>,
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

/// The status of all provided parameters for this parameter provider
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterStatusDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<ParameterEntity>,
    /// Indicates the status of the parameter, compared to the existing parameter context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteRequestEntity {
    #[serde(rename = "copyResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_response: Option<CopyResponseEntity>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteResponseEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<FlowDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerDto {
    /// The number of flowFiles this peer holds.
    #[serde(rename = "flowFileCount")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeersEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peers: Option<Vec<PeerDto>>,
}

/// The permissions for this component.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsDto {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// Indicates whether the user can write a given resource.
    #[serde(rename = "canWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_write: Option<bool>,
}

/// The output ports available to received data from the NiFi.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortDto {
    /// Whether this port can be accessed remotely via Site-to-Site protocol.
    #[serde(rename = "allowRemoteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_access: Option<bool>,
    /// The comments for the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of tasks that should be concurrently scheduled for the port.
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Specifies how the Port functions
    #[serde(rename = "portFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The state of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Whether the port has incoming or output connections to a remote NiFi. This is only applicable when the port is allowed to be accessed remotely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
    /// The type of port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from this port. These validation errors represent the problems with the port that must be resolved before it can be started.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// The output ports in this flow.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortEntity {
    /// Whether this port can be accessed remotely via Site-to-Site protocol.
    #[serde(rename = "allowRemoteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_access: Option<bool>,
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<PortDto>,
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
    #[serde(rename = "portType")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the Port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// The status of the port.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<PortStatusSnapshotDto>,
    /// The id of the parent process group of the port.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodePortStatusSnapshotDto>>,
    /// The run status of the port.
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// The time the status for the process group was last refreshed.
    #[serde(rename = "statsLastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// Whether the port has incoming or outgoing connections to a remote NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
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
pub struct PortStatusSnapshotDto {
    /// The active thread count for the port.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of hte FlowFiles that have been accepted in the last 5 minutes.
    #[serde(rename = "bytesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The number of bytes that have been processed in the last 5 minutes.
    #[serde(rename = "bytesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The number of FlowFiles that have been accepted in the last 5 minutes.
    #[serde(rename = "flowFilesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles that have been processed in the last 5 minutes.
    #[serde(rename = "flowFilesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The id of the parent process group of the port.
    #[serde(rename = "groupId")]
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
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// Whether the port has incoming or outgoing connections to a remote NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
}

/// The status of all output ports in the process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "portStatusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_status_snapshot: Option<PortStatusSnapshotDto>,
}

/// The position of a component on the graph
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousValueDto {
    /// The previous value.
    #[serde(rename = "previousValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,
    /// The timestamp when the value was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The user who changed the previous value.
    #[serde(rename = "userIdentity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<String>,
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
pub struct ProcessGroupDto {
    /// The number of active remote ports in the process group.
    #[serde(rename = "activeRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The comments for the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<FlowSnippetDto>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureDataSizeThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureObjectThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// The default FlowFile Expiration for this Process Group.
    #[serde(rename = "defaultFlowFileExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_flow_file_expiration: Option<String>,
    /// The number of disabled components in the process group.
    #[serde(rename = "disabledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The Execution Engine that should be used to run the flow represented by this Process Group.
    #[serde(rename = "executionEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_engine: Option<String>,
    /// The FlowFile Concurrency for this Process Group.
    #[serde(rename = "flowfileConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flowfile_concurrency: Option<String>,
    /// The Outbound Policy that is used for determining how FlowFiles should be transferred out of the Process Group.
    #[serde(rename = "flowfileOutboundPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flowfile_outbound_policy: Option<String>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of inactive remote ports in the process group.
    #[serde(rename = "inactiveRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports in the process group.
    #[serde(rename = "inputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The number of invalid components in the process group.
    #[serde(rename = "invalidCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The number of local input ports in the process group.
    #[serde(rename = "localInputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_input_port_count: Option<i32>,
    /// The number of local output ports in the process group.
    #[serde(rename = "localOutputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_output_port_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the process group.
    #[serde(rename = "locallyModifiedAndStaleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of locally modified versioned process groups in the process group.
    #[serde(rename = "locallyModifiedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_count: Option<i32>,
    /// The log file suffix for this Process Group for dedicated logging.
    #[serde(rename = "logFileSuffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_suffix: Option<String>,
    /// The maximum number of concurrent tasks to use when running the flow using the Stateless Engine
    #[serde(rename = "maxConcurrentTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_tasks: Option<i32>,
    /// The name of the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of output ports in the process group.
    #[serde(rename = "outputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    #[serde(rename = "parameterContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The number of public input ports in the process group.
    #[serde(rename = "publicInputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_input_port_count: Option<i32>,
    /// The number of public output ports in the process group.
    #[serde(rename = "publicOutputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_output_port_count: Option<i32>,
    /// The number of running components in this process group.
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the process group.
    #[serde(rename = "staleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_count: Option<i32>,
    /// The maximum amount of time that the flow can be run using the Stateless Engine before the flow times out
    #[serde(rename = "statelessFlowTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_flow_timeout: Option<String>,
    /// If the Process Group is configured to run in using the Stateless Engine, represents the current state. Otherwise, will be STOPPED.
    #[serde(rename = "statelessGroupScheduledState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_group_scheduled_state: Option<String>,
    /// The number of stopped components in the process group.
    #[serde(rename = "stoppedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the process group that are unable to sync to a registry.
    #[serde(rename = "syncFailureCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_failure_count: Option<i32>,
    /// The number of up to date versioned process groups in the process group.
    #[serde(rename = "upToDateCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to_date_count: Option<i32>,
    #[serde(rename = "versionControlInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupEntity {
    /// The number of active remote ports in the process group.
    #[serde(rename = "activeRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ProcessGroupDto>,
    /// The number of disabled components in the process group.
    #[serde(rename = "disabledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of inactive remote ports in the process group.
    #[serde(rename = "inactiveRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports in the process group.
    #[serde(rename = "inputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The number of invalid components in the process group.
    #[serde(rename = "invalidCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The number of local input ports in the process group.
    #[serde(rename = "localInputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_input_port_count: Option<i32>,
    /// The number of local output ports in the process group.
    #[serde(rename = "localOutputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_output_port_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the process group.
    #[serde(rename = "locallyModifiedAndStaleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of locally modified versioned process groups in the process group.
    #[serde(rename = "locallyModifiedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_count: Option<i32>,
    /// The number of output ports in the process group.
    #[serde(rename = "outputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    #[serde(rename = "parameterContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// Determines the process group update strategy
    #[serde(rename = "processGroupUpdateStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_update_strategy: Option<String>,
    /// The number of public input ports in the process group.
    #[serde(rename = "publicInputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_input_port_count: Option<i32>,
    /// The number of public output ports in the process group.
    #[serde(rename = "publicOutputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_output_port_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The number of running components in this process group.
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the process group.
    #[serde(rename = "staleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProcessGroupStatusDto>,
    /// The number of stopped components in the process group.
    #[serde(rename = "stoppedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the process group that are unable to sync to a registry.
    #[serde(rename = "syncFailureCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_failure_count: Option<i32>,
    /// The number of up to date versioned process groups in the process group.
    #[serde(rename = "upToDateCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to_date_count: Option<i32>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "versionedFlowSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
    /// The current state of the Process Group, as it relates to the Versioned Flow
    #[serde(rename = "versionedFlowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_state: Option<String>,
}

#[non_exhaustive]
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
    #[serde(rename = "lastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed: Option<String>,
    #[serde(rename = "parameterContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
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
pub struct ProcessGroupImportEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionedFlowSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}

/// The Process Group that the component belongs to
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestDto {
    /// Whether or not this request has completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// An explanation of why this request failed, or null if this request has not failed
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The last time this request was updated.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The percentage complete for the request, between 0 and 100
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The unique ID of the Process Group being updated
    #[serde(rename = "processGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The unique ID of this request.
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The state of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The URI for future requests to this drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestEntity {
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ProcessGroupReplaceRequestDto>,
    #[serde(rename = "versionedFlowSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}

/// The status of the process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ProcessGroupStatusSnapshotDto>,
    /// The ID of the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status reported by each node in the cluster. If the NiFi instance is a standalone instance, rather than a clustered instance, this value may be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeProcessGroupStatusSnapshotDto>>,
    /// The time the status for the process group was last refreshed.
    #[serde(rename = "statsLastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
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

/// The process group status snapshot from the node.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusSnapshotDto {
    /// The active thread count for this process group.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The number of bytes that have come into this ProcessGroup in the last 5 minutes
    #[serde(rename = "bytesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The number of bytes transferred out of this ProcessGroup in the last 5 minutes
    #[serde(rename = "bytesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The number of bytes that are queued up in this ProcessGroup right now
    #[serde(rename = "bytesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_queued: Option<i64>,
    /// The number of bytes read by components in this ProcessGroup in the last 5 minutes
    #[serde(rename = "bytesRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_read: Option<i64>,
    /// The number of bytes received from external sources by components within this ProcessGroup in the last 5 minutes
    #[serde(rename = "bytesReceived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<i64>,
    /// The number of bytes sent to an external sink by components within this ProcessGroup in the last 5 minutes
    #[serde(rename = "bytesSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<i64>,
    /// The number of bytes transferred in this ProcessGroup in the last 5 minutes
    #[serde(rename = "bytesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    /// The number of bytes written by components in this ProcessGroup in the last 5 minutes
    #[serde(rename = "bytesWritten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i64>,
    /// The status of all connections in the process group.
    #[serde(rename = "connectionStatusSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status_snapshots: Option<Vec<ConnectionStatusSnapshotEntity>>,
    /// The number of FlowFiles that have come into this ProcessGroup in the last 5 minutes
    #[serde(rename = "flowFilesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles transferred out of this ProcessGroup in the last 5 minutes
    #[serde(rename = "flowFilesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The number of FlowFiles that are queued up in this ProcessGroup right now
    #[serde(rename = "flowFilesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_queued: Option<i32>,
    /// The number of FlowFiles received from external sources by components within this ProcessGroup in the last 5 minutes
    #[serde(rename = "flowFilesReceived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_received: Option<i32>,
    /// The number of FlowFiles sent to an external sink by components within this ProcessGroup in the last 5 minutes
    #[serde(rename = "flowFilesSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_sent: Option<i32>,
    /// The number of FlowFiles transferred in this ProcessGroup in the last 5 minutes
    #[serde(rename = "flowFilesTransferred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_transferred: Option<i32>,
    /// The id of the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input count/size for the process group in the last 5 minutes (pretty printed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The status of all input ports in the process group.
    #[serde(rename = "inputPortStatusSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    /// The name of this process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The output count/size for the process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The status of all output ports in the process group.
    #[serde(rename = "outputPortStatusSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    /// The status of all process groups in the process group.
    #[serde(rename = "processGroupStatusSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_status_snapshots: Option<Vec<ProcessGroupStatusSnapshotEntity>>,
    #[serde(rename = "processingNanos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_nanos: Option<i64>,
    #[serde(rename = "processingPerformanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    /// The status of all processors in the process group.
    #[serde(rename = "processorStatusSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_status_snapshots: Option<Vec<ProcessorStatusSnapshotEntity>>,
    /// The count/size that is queued in the the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    /// The count that is queued for the process group.
    #[serde(rename = "queuedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_count: Option<String>,
    /// The size that is queued for the process group.
    #[serde(rename = "queuedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_size: Option<String>,
    /// The number of bytes read in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    /// The count/size sent to the process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<String>,
    /// The status of all remote process groups in the process group.
    #[serde(rename = "remoteProcessGroupStatusSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_status_snapshots: Option<Vec<RemoteProcessGroupStatusSnapshotEntity>>,
    /// The count/size sent from this process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
    /// The current number of active threads for the Process Group, when running in Stateless mode.
    #[serde(rename = "statelessActiveThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_active_thread_count: Option<i32>,
    /// The number of threads currently terminated for the process group.
    #[serde(rename = "terminatedThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_thread_count: Option<i32>,
    /// The count/size transferred to/from queues in the process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred: Option<String>,
    /// The current state of the Process Group, as it relates to the Versioned Flow
    #[serde(rename = "versionedFlowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_state: Option<String>,
    /// The number of bytes written in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written: Option<String>,
}

/// The status of all process groups in the process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "processGroupStatusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupUploadEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "flowSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_snapshot: Option<RegisteredFlowSnapshot>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "positionDTO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_d_t_o: Option<PositionDto>,
    #[serde(rename = "revisionDTO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_d_t_o: Option<RevisionDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupsEntity {
    #[serde(rename = "processGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
}

/// Represents the processor's processing performance.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingPerformanceStatusDto {
    /// The number of nanoseconds has spent to read content in the last 5 minutes.
    #[serde(rename = "contentReadDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_read_duration: Option<i64>,
    /// The number of nanoseconds has spent to write content in the last 5 minutes.
    #[serde(rename = "contentWriteDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_write_duration: Option<i64>,
    /// The number of nanoseconds has spent on CPU usage in the last 5 minutes.
    #[serde(rename = "cpuDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_duration: Option<i64>,
    /// The number of nanoseconds has spent running garbage collection in the last 5 minutes.
    #[serde(rename = "garbageCollectionDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garbage_collection_duration: Option<i64>,
    /// The unique ID of the process group that the Processor belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The number of nanoseconds has spent running to commit sessions the last 5 minutes.
    #[serde(rename = "sessionCommitDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_commit_duration: Option<i64>,
}

/// The configuration details for the processor. These details will be included in a response if the verbose flag is included in a request.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfigDto {
    /// The annotation data for the processor used to relay configuration between a custom UI and the procesosr.
    #[serde(rename = "annotationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated.
    #[serde(rename = "autoTerminatedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminated_relationships: Option<Vec<String>>,
    /// Determines whether the FlowFile should be penalized or the processor should be yielded between retries. Possible returned values: PENALIZE_FLOWFILE, YIELD_PROCESSOR. See BackoffMechanism.class for more details.
    #[serde(rename = "backoffMechanism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_mechanism: Option<String>,
    /// The level at which the processor will report bulletins.
    #[serde(rename = "bulletinLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    /// The comments for the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored.
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The URL for the processor's custom configuration UI if applicable.
    #[serde(rename = "customUiUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// Maps default values for concurrent tasks for each applicable scheduling strategy.
    #[serde(rename = "defaultConcurrentTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_concurrent_tasks: Option<std::collections::HashMap<String, Option<String>>>,
    /// Maps default values for scheduling period for each applicable scheduling strategy.
    #[serde(rename = "defaultSchedulingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    /// Descriptors for the processor's properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Indicates the node where the process will execute.
    #[serde(rename = "executionNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node: Option<String>,
    /// Whether the processor is loss tolerant.
    #[serde(rename = "lossTolerant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_tolerant: Option<bool>,
    /// Maximum amount of time to be waited during a retry period.
    #[serde(rename = "maxBackoffPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_backoff_period: Option<String>,
    /// The amount of time that is used when the process penalizes a flowfile.
    #[serde(rename = "penaltyDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_duration: Option<String>,
    /// The properties for the processor. Properties whose value is not set will only contain the property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// All the relationships should be retried.
    #[serde(rename = "retriedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_relationships: Option<Vec<String>>,
    /// Overall number of retries.
    #[serde(rename = "retryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    /// The run duration for the processor in milliseconds.
    #[serde(rename = "runDurationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_duration_millis: Option<i64>,
    /// The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy.
    #[serde(rename = "schedulingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// Indicates how the processor should be scheduled to run.
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// Set of sensitive dynamic property names
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The amount of time that must elapse before this processor is scheduled again after yielding.
    #[serde(rename = "yieldDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
}

/// A description of how to configure the Processor to perform the task described in the use case
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfiguration {
    /// A description of how the Processor should be configured in order to accomplish the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// The fully qualified classname of the Processor that should be used to accomplish the use case
    #[serde(rename = "processorClassName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_class_name: Option<String>,
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
    #[serde(rename = "executionNodeRestricted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node_restricted: Option<bool>,
    /// Whether the underlying extension is missing.
    #[serde(rename = "extensionMissing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input requirement for this processor.
    #[serde(rename = "inputRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<String>,
    /// Whether the processor has multiple versions available.
    #[serde(rename = "multipleVersionsAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the processor persists state.
    #[serde(rename = "persistsState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    /// The physical state of the processor, including transition states
    #[serde(rename = "physicalState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The available relationships that the processor currently supports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<RelationshipDto>>,
    /// Whether the processor requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// The state of the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Styles for the processor (background-color : #eee).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the processor supports batching. This makes the run duration settings available.
    #[serde(rename = "supportsBatching")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_batching: Option<bool>,
    /// Whether the processor supports parallel processing.
    #[serde(rename = "supportsParallelProcessing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_parallel_processing: Option<bool>,
    /// Whether the processor supports sensitive dynamic properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The validation errors for the processor. These validation errors represent the problems with the processor that must be resolved before it can be started.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Processor is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Processor is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ProcessorDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input requirement for this processor.
    #[serde(rename = "inputRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<String>,
    #[serde(rename = "operatePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    /// The physical state of the processor, including transition states
    #[serde(rename = "physicalState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_state: Option<String>,
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

/// The details of a Processor's run status
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusDetailsDto {
    /// The current number of threads that the processor is currently using
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The ID of the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The run status of the processor
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// The processor's validation errors
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusDetailsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(rename = "runStatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status_details: Option<ProcessorRunStatusDetailsDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the Processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ProcessorStatusSnapshotDto>,
    /// The unique ID of the process group that the Processor belongs to
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The unique ID of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeProcessorStatusSnapshotDto>>,
    /// The run status of the Processor
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// The timestamp of when the stats were last refreshed
    #[serde(rename = "statsLastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// The type of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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

/// The processor status snapshot from the node.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusSnapshotDto {
    /// The number of threads currently executing in the processor.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles that have been accepted in the last 5 minutes
    #[serde(rename = "bytesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The size of the FlowFiles transferred to a Connection in the last 5 minutes
    #[serde(rename = "bytesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The number of bytes read by this Processor in the last 5 mintues
    #[serde(rename = "bytesRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_read: Option<i64>,
    /// The number of bytes written by this Processor in the last 5 minutes
    #[serde(rename = "bytesWritten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i64>,
    /// Indicates the node where the process will execute.
    #[serde(rename = "executionNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node: Option<String>,
    /// The number of FlowFiles that have been accepted in the last 5 minutes
    #[serde(rename = "flowFilesIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles transferred to a Connection in the last 5 minutes
    #[serde(rename = "flowFilesOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The id of the parent process group to which the processor belongs.
    #[serde(rename = "groupId")]
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
    #[serde(rename = "processingPerformanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    /// The number of bytes read in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    /// The state of the processor.
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// The number of times this Processor has run in the last 5 minutes
    #[serde(rename = "taskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i32>,
    /// The total number of task this connectable has completed over the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<String>,
    /// The total duration of all tasks for this connectable over the last 5 minutes.
    #[serde(rename = "tasksDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks_duration: Option<String>,
    /// The number of nanoseconds that this Processor has spent running in the last 5 minutes
    #[serde(rename = "tasksDurationNanos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks_duration_nanos: Option<i64>,
    /// The number of threads currently terminated for the processor.
    #[serde(rename = "terminatedThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_thread_count: Option<i32>,
    /// The type of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The number of bytes written in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written: Option<String>,
}

/// The status of all processors in the process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "processorStatusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_status_snapshot: Option<ProcessorStatusSnapshotDto>,
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
pub struct ProcessorsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorsRunStatusDetailsEntity {
    #[serde(rename = "runStatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status_details: Option<Vec<ProcessorRunStatusDetailsEntity>>,
}

/// A list of the allowable values for the property
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyAllowableValue {
    /// The description of the value, e.g., the behavior it produces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The display name of the value, if different from the internal value
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The internal value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The dependencies that this property has on other properties
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependency {
    /// The values that satisfy the dependency
    #[serde(rename = "dependentValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_values: Option<Vec<String>>,
    /// The name of the property that is depended upon
    #[serde(rename = "propertyDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_display_name: Option<String>,
    /// The name of the property that is depended upon
    #[serde(rename = "propertyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
}

/// A list of dependencies that must be met in order for this Property to be relevant. If any of these dependencies is not met, the property described by this Property Descriptor is not relevant.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependencyDto {
    /// The values for the property that satisfies the dependency, or null if the dependency is satisfied by the presence of any value for the associated property name
    #[serde(rename = "dependentValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_values: Option<Vec<String>>,
    /// The name of the property that is being depended upon
    #[serde(rename = "propertyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
}

/// Descriptions of configuration properties applicable to this component.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor {
    /// A list of the allowable values for the property
    #[serde(rename = "allowableValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowable_values: Option<Vec<PropertyAllowableValue>>,
    /// The default value if a user-set value is not specified
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The dependencies that this property has on other properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<PropertyDependency>>,
    /// The description of what the property does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The display name of the property key, if different from the name
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether or not the descriptor is for a dynamically added property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    /// The scope of expression language supported by this property
    #[serde(rename = "expressionLanguageScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope: Option<String>,
    /// The description of the expression language scope supported by this property
    #[serde(rename = "expressionLanguageScopeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope_description: Option<String>,
    #[serde(rename = "listenPortDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port_definition: Option<PropertyListenPortDefinition>,
    /// The name of the property key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether or not  the property is required for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "resourceDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition: Option<PropertyResourceDefinition>,
    /// Whether or not  the value of the property is considered sensitive (e.g., passwords and keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(rename = "typeProvidedByValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_provided_by_value: Option<DefinedType>,
    /// A regular expression that can be used to validate the value of this property
    #[serde(rename = "validRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_regex: Option<String>,
    /// Name of the validator used for this property descriptor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<String>,
}

/// The descriptors for the reporting tasks properties.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorDto {
    /// Allowable values for the property. If empty then the allowed values are not constrained.
    #[serde(rename = "allowableValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowable_values: Option<Vec<AllowableValueEntity>>,
    /// The default value for the property.
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// A list of dependencies that must be met in order for this Property to be relevant. If any of these dependencies is not met, the property described by this Property Descriptor is not relevant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<PropertyDependencyDto>>,
    /// The description for the property. Used to relay additional details to a user or provide a mechanism of documenting intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The human readable name for the property.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether the property is dynamic (user-defined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    /// Scope of the Expression Language evaluation for the property.
    #[serde(rename = "expressionLanguageScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope: Option<String>,
    /// If the property identifies a controller service this returns the fully qualified type.
    #[serde(rename = "identifiesControllerService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifies_controller_service: Option<String>,
    #[serde(rename = "identifiesControllerServiceBundle")]
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
    #[serde(rename = "supportsEl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_el: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorEntity {
    pub property_descriptor: Option<PropertyDescriptorDto>,
}

/// The history for the properties of the component.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyHistoryDto {
    /// Previous values for a given property.
    #[serde(rename = "previousValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_values: Option<Vec<PreviousValueDto>>,
}

/// Indicates that this property defines a listen port
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyListenPortDefinition {
    /// The application protocols that this listen port could support (if any)
    #[serde(rename = "applicationProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocols: Option<Vec<String>>,
    /// The transport protocol used by this listen port
    #[serde(rename = "transportProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
}

/// Indicates that this property references external resources
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyResourceDefinition {
    /// The cardinality of the resource definition (i.e. single or multiple)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<String>,
    /// The types of resources that can be referenced
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceDto {
    /// The timestamp when the query will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Whether the query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The id of the provenance query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The current percent complete.
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ProvenanceRequestDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<ProvenanceResultsDto>,
    /// The timestamp when the query was submitted.
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for this query. Used for obtaining/deleting the request at a later time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEntity {
    pub provenance: Option<ProvenanceDto>,
}

/// The provenance events that matched the search criteria.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEventDto {
    /// The alternate identifier uri for the fileflow for the event.
    #[serde(rename = "alternateIdentifierUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_identifier_uri: Option<String>,
    /// The attributes of the flowfile for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeDto>>,
    /// The child uuids for the event.
    #[serde(rename = "childUuids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_uuids: Option<Vec<String>>,
    /// The label for the node where the event originated.
    #[serde(rename = "clusterNodeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The identifier for the node where the event originated.
    #[serde(rename = "clusterNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The id of the component that generated the event.
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The name of the component that generated the event.
    #[serde(rename = "componentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// The type of the component that generated the event.
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// Whether the input and output content claim is the same.
    #[serde(rename = "contentEqual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_equal: Option<bool>,
    /// The event details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The event duration in milliseconds.
    #[serde(rename = "eventDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_duration: Option<i64>,
    /// The event id. This is a one up number thats unique per node.
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    /// The timestamp of the event.
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// Event Timestamp formatted using ISO8601
    #[serde(rename = "eventTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<String>,
    /// The type of the event.
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// The size of the flowfile for the event.
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    /// The size of the flowfile in bytes for the event.
    #[serde(rename = "fileSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<i64>,
    /// The uuid of the flowfile for the event.
    #[serde(rename = "flowFileUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_uuid: Option<String>,
    /// The id of the group that the component resides in. If the component is no longer in the flow, the group id will not be set.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The event uuid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the input content is still available.
    #[serde(rename = "inputContentAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_available: Option<bool>,
    /// The container in which the input content claim lives.
    #[serde(rename = "inputContentClaimContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_container: Option<String>,
    /// The file size of the input content claim formatted.
    #[serde(rename = "inputContentClaimFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_file_size: Option<String>,
    /// The file size of the intput content claim in bytes.
    #[serde(rename = "inputContentClaimFileSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the input content claim.
    #[serde(rename = "inputContentClaimIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_identifier: Option<String>,
    /// The offset into the input content claim where the flowfiles content begins.
    #[serde(rename = "inputContentClaimOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_offset: Option<i64>,
    /// The section in which the input content claim lives.
    #[serde(rename = "inputContentClaimSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_content_claim_section: Option<String>,
    /// The duration since the lineage began, in milliseconds.
    #[serde(rename = "lineageDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// Whether the output content is still available.
    #[serde(rename = "outputContentAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_available: Option<bool>,
    /// The container in which the output content claim lives.
    #[serde(rename = "outputContentClaimContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_container: Option<String>,
    /// The file size of the output content claim formatted.
    #[serde(rename = "outputContentClaimFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_file_size: Option<String>,
    /// The file size of the output content claim in bytes.
    #[serde(rename = "outputContentClaimFileSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_file_size_bytes: Option<i64>,
    /// The identifier of the output content claim.
    #[serde(rename = "outputContentClaimIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_identifier: Option<String>,
    /// The offset into the output content claim where the flowfiles content begins.
    #[serde(rename = "outputContentClaimOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_offset: Option<i64>,
    /// The section in which the output content claim lives.
    #[serde(rename = "outputContentClaimSection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_content_claim_section: Option<String>,
    /// The parent uuids for the event.
    #[serde(rename = "parentUuids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuids: Option<Vec<String>>,
    /// The relationship to which the flowfile was routed if the event is of type ROUTE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// Whether or not replay is available.
    #[serde(rename = "replayAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_available: Option<bool>,
    /// Explanation as to why replay is unavailable.
    #[serde(rename = "replayExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_explanation: Option<String>,
    /// The identifier of the queue/connection from which the flowfile was pulled to genereate this event. May be null if the queue/connection is unknown or the flowfile was generated from this event.
    #[serde(rename = "sourceConnectionIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connection_identifier: Option<String>,
    /// The source system flowfile id.
    #[serde(rename = "sourceSystemFlowFileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_system_flow_file_id: Option<String>,
    /// The source/destination system uri if the event was a RECEIVE/SEND.
    #[serde(rename = "transitUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEventEntity {
    pub provenance_event: Option<ProvenanceEventDto>,
}

/// The links between the nodes in the lineage.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceLinkDto {
    /// The flowfile uuid that traversed the link.
    #[serde(rename = "flowFileUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_uuid: Option<String>,
    /// The timestamp of this link in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis: Option<i64>,
    /// The source node id of the link.
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The target node id of the link.
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// The timestamp of the link (based on the destination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// The nodes in the lineage.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceNodeDto {
    /// The uuid of the childrent flowfiles of the provenance event.
    #[serde(rename = "childUuids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_uuids: Option<Vec<String>>,
    /// The identifier of the node that this event/flowfile originated from.
    #[serde(rename = "clusterNodeIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_identifier: Option<String>,
    /// If the type is EVENT, this is the type of the component that generated the event.
    #[serde(rename = "componentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// If the type is EVENT, this is the type of event.
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// The uuid of the flowfile associated with the provenance event.
    #[serde(rename = "flowFileUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_uuid: Option<String>,
    /// The id of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The timestamp of the node in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis: Option<i64>,
    /// The uuid of the parent flowfiles of the provenance event.
    #[serde(rename = "parentUuids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuids: Option<Vec<String>>,
    /// The timestamp of the node formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The type of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceOptionsDto {
    /// The available searchable field for the NiFi.
    #[serde(rename = "searchableFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_fields: Option<Vec<ProvenanceSearchableFieldDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceOptionsEntity {
    pub provenance_options: Option<ProvenanceOptionsDto>,
}

/// The provenance request.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceRequestDto {
    /// The id of the node in the cluster where this provenance originated.
    #[serde(rename = "clusterNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The latest event time to include in the query.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Whether or not incremental results are returned. If false, provenance events are only returned once the query completes. This property is true by default.
    #[serde(rename = "incrementalResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_results: Option<bool>,
    /// The maximum number of results to include.
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The maximum file size to include in the query.
    #[serde(rename = "maximumFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_file_size: Option<String>,
    /// The minimum file size to include in the query.
    #[serde(rename = "minimumFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_file_size: Option<String>,
    /// The search terms used to perform the search.
    #[serde(rename = "searchTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_terms: Option<std::collections::HashMap<String, Option<ProvenanceSearchValueDto>>>,
    /// The earliest event time to include in the query.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Whether or not to summarize provenance events returned. This property is false by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summarize: Option<bool>,
}

/// The provenance results.
#[non_exhaustive]
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
    #[serde(rename = "oldestEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_event: Option<String>,
    /// The provenance events that matched the search criteria.
    #[serde(rename = "provenanceEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
    /// The time offset of the server that's used for event time.
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
    /// The total number of results formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
    /// The total number of results.
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// The search terms used to perform the search.
#[non_exhaustive]
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
#[non_exhaustive]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// The size of the queue
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSizeDto {
    /// The size of objects in a queue.
    #[serde(rename = "byteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<i64>,
    /// The count of objects in a queue.
    #[serde(rename = "objectCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i32>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "bucketIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_identifier: Option<String>,
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "createdTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "lastModifiedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FlowRegistryPermissions>,
    #[serde(rename = "versionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_count: Option<i64>,
    #[serde(rename = "versionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<RegisteredFlowVersionInfo>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<FlowRegistryBucket>,
    #[serde(rename = "externalControllerServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_controller_services:
        Option<std::collections::HashMap<String, Option<ExternalControllerServiceReference>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<RegisteredFlow>,
    #[serde(rename = "flowContents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_contents: Option<VersionedProcessGroup>,
    #[serde(rename = "flowEncodingVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_encoding_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    #[serde(rename = "parameterContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    #[serde(rename = "parameterProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    #[serde(rename = "snapshotMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshotMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(rename = "bucketIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "flowIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_identifier: Option<String>,
    #[serde(rename = "flowName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    #[serde(rename = "registryIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_identifier: Option<String>,
    #[serde(rename = "registryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowVersionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// The supported relationships for this processor.
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDto {
    /// Whether or not flowfiles sent to this relationship should auto terminate.
    #[serde(rename = "autoTerminate")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemotePortRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the RemotePort.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// The contents of the remote process group. Will contain available input/output ports.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupContentsDto {
    /// The input ports to which data can be sent.
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<RemoteProcessGroupPortDto>>,
    /// The output ports from which data can be retrieved.
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<RemoteProcessGroupPortDto>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupDto {
    /// The number of active remote input ports.
    #[serde(rename = "activeRemoteInputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_input_port_count: Option<i32>,
    /// The number of active remote output ports.
    #[serde(rename = "activeRemoteOutputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_output_port_count: Option<i32>,
    /// Any remote authorization issues for the remote process group.
    #[serde(rename = "authorizationIssues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_issues: Option<Vec<String>>,
    /// The comments for the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The time period used for the timeout when communicating with the target.
    #[serde(rename = "communicationsTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<RemoteProcessGroupContentsDto>,
    /// The timestamp when this remote process group was last refreshed.
    #[serde(rename = "flowRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_refreshed: Option<String>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of inactive remote input ports.
    #[serde(rename = "inactiveRemoteInputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_input_port_count: Option<i32>,
    /// The number of inactive remote output ports.
    #[serde(rename = "inactiveRemoteOutputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_output_port_count: Option<i32>,
    /// The number of remote input ports currently available on the target.
    #[serde(rename = "inputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier.
    #[serde(rename = "localNetworkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_network_interface: Option<String>,
    /// The name of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of remote output ports currently available on the target.
    #[serde(rename = "outputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(rename = "proxyHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_host: Option<String>,
    #[serde(rename = "proxyPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,
    #[serde(rename = "proxyPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i32>,
    #[serde(rename = "proxyUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_user: Option<String>,
    /// Whether the target is running securely.
    #[serde(rename = "targetSecure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_secure: Option<bool>,
    /// The target URI of the remote process group. If target uri is not set, but uris are set, then returns the first url in the urls. If neither target uri nor uris are set, then returns null.
    #[serde(rename = "targetUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    /// The target URI of the remote process group. If target uris is not set but target uri is set, then returns a collection containing the single target uri. If neither target uris nor uris are set, then returns null.
    #[serde(rename = "targetUris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uris: Option<String>,
    /// Whether the remote process group is actively transmitting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
    #[serde(rename = "transportProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
    /// The validation errors for the remote process group.
    /// These validation errors represent the problems with the remote process group that must be resolved before it can transmit.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
    /// When yielding, this amount of time must elapse before the remote process group is scheduled again.
    #[serde(rename = "yieldDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<RemoteProcessGroupDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of remote input ports currently available on the target.
    #[serde(rename = "inputPortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    #[serde(rename = "operatePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    /// The number of remote output ports currently available on the target.
    #[serde(rename = "outputPortCount")]
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
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupPortDto {
    #[serde(rename = "batchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_settings: Option<BatchSettingsDto>,
    /// The comments as configured on the target port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of task that may transmit flowfiles to the target port concurrently.
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// Whether the port has either an incoming or outgoing connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// Whether the target port exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    /// The id of the remote process group that the port resides in.
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the target port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of the target port.
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// Whether the target port is running.
    #[serde(rename = "targetRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_running: Option<bool>,
    /// Whether the remote port is configured for transmission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
    /// Whether the flowfiles are compressed when sent to the target port.
    #[serde(rename = "useCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_compression: Option<bool>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupPortEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
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
    #[serde(rename = "remoteProcessGroupPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_port: Option<RemoteProcessGroupPortDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// The status of the remote process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
    /// The unique ID of the process group that the Processor belongs to
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The unique ID of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeRemoteProcessGroupStatusSnapshotDto>>,
    /// The time the status for the process group was last refreshed.
    #[serde(rename = "statsLastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// The URI of the target system.
    #[serde(rename = "targetUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    /// The transmission status of the remote process group.
    #[serde(rename = "transmissionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_status: Option<String>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
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

/// The remote process group status snapshot from the node.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotDto {
    /// The number of active threads for the remote process group.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles received from the remote process group in the last 5 minutes.
    #[serde(rename = "bytesReceived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<i64>,
    /// The size of the FlowFiles sent to the remote process group in the last 5 minutes.
    #[serde(rename = "bytesSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<i64>,
    /// The number of FlowFiles received from the remote process group in the last 5 minutes.
    #[serde(rename = "flowFilesReceived")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_received: Option<i32>,
    /// The number of FlowFiles sent to the remote process group in the last 5 minutes.
    #[serde(rename = "flowFilesSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_sent: Option<i32>,
    /// The id of the parent process group the remote process group resides in.
    #[serde(rename = "groupId")]
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
    #[serde(rename = "targetUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    /// The transmission status of the remote process group.
    #[serde(rename = "transmissionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_status: Option<String>,
}

/// The status of all remote process groups in the process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "remoteProcessGroupStatusSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupsEntity {
    #[serde(rename = "remoteProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventRequestEntity {
    /// The UUID of the component whose last event should be replayed.
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// Which nodes are to replay their last provenance event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventResponseEntity {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ReplayLastEventSnapshotDto>,
    /// The UUID of the component whose last event should be replayed.
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The node-wise results
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeReplayLastEventSnapshotDto>>,
    /// Which nodes were requested to replay their last provenance event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<String>,
}

/// The snapshot from the node
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventSnapshotDto {
    /// Whether or not an event was available. This may not be populated if there was a failure.
    #[serde(rename = "eventAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_available: Option<bool>,
    /// The IDs of the events that were successfully replayed
    #[serde(rename = "eventsReplayed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_replayed: Option<Vec<i64>>,
    /// If unable to replay an event, specifies why the event could not be replayed
    #[serde(rename = "failureExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_explanation: Option<String>,
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
pub struct ReportingTaskDto {
    /// The number of active threads for the reporting task.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The annotation data for the repoting task. This is how the custom UI relays configuration to the reporting task.
    #[serde(rename = "annotationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The comments of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The URL for the custom configuration UI for the reporting task.
    #[serde(rename = "customUiUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// The default scheduling period for the different scheduling strategies.
    #[serde(rename = "defaultSchedulingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the reporting task has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The descriptors for the reporting tasks properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    #[serde(rename = "extensionMissing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the reporting task has multiple versions available.
    #[serde(rename = "multipleVersionsAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the reporting task persists state.
    #[serde(rename = "persistsState")]
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
    #[serde(rename = "schedulingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// The scheduling strategy that determines how the schedulingPeriod value should be interpreted.
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// Set of sensitive dynamic property names
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Whether the reporting task supports sensitive dynamic properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The fully qualified type of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the reporting task. These validation errors represent the problems with the reporting task that must be resolved before it can be scheduled to run.
    #[serde(rename = "validationErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Reporting Task is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Reporting Task is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ReportingTaskDto>,
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
    pub status: Option<ReportingTaskStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the ReportingTask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// The status for this ReportingTask.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskStatusDto {
    /// The number of active threads for the component.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The run status of this ReportingTask
    #[serde(rename = "runStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<String>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid)
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
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

/// The required permission necessary for this restriction.
#[non_exhaustive]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimDetailsDto {
    /// Whether or not the Resource Claim is awaiting destruction
    #[serde(rename = "awaitingDestruction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaiting_destruction: Option<bool>,
    /// The number of FlowFiles that have a claim to the Resource
    #[serde(rename = "claimantCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimant_count: Option<i32>,
    /// The container of the Content Repository in which the Resource Claim exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// The identifier of the Resource Claim
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Whether or not the Resource Claim is in use
    #[serde(rename = "inUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    /// The section of the Content Repository in which the Resource Claim exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    /// Whether or not the Resource Claim can still have more data written to it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writable: Option<bool>,
}

#[non_exhaustive]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceDto>>,
}

/// Explicit restrictions that indicate a require permission to use the component
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    /// The explanation of this restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// The permission required for this restriction
    #[serde(rename = "requiredPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permission: Option<String>,
}

/// The revision of the Process Group
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionDto {
    /// A client identifier used to make a request.
    /// By including a client identifier, the API can allow multiple requests without needing the current revision.
    /// Due to the asynchronous nature of requests/responses this was implemented to allow the client to make numerous requests without having to wait for the previous response to come back
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The user that last modified the flow.
    #[serde(rename = "lastModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modifier: Option<String>,
    /// NiFi employs an optimistic locking strategy where the client must include a revision in their request when performing an update.
    /// In a response to a mutable flow request, this field represents the updated base version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunStatusDetailsRequestEntity {
    /// The IDs of all processors whose run status details should be provided
    #[serde(rename = "processorIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_ids: Option<Vec<String>>,
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

/// Scheduling defaults for components defined in this manifest
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchedulingDefaults {
    /// The default concurrent tasks for each scheduling strategy
    #[serde(rename = "defaultConcurrentTasksBySchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    /// The default concurrent tasks
    #[serde(rename = "defaultMaxConcurrentTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_max_concurrent_tasks: Option<String>,
    /// The default run duration in nano-seconds
    #[serde(rename = "defaultRunDurationNanos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_duration_nanos: Option<i64>,
    /// The default scheduling period in milliseconds
    #[serde(rename = "defaultSchedulingPeriodMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period_millis: Option<i64>,
    /// The default scheduling period for each scheduling strategy
    #[serde(rename = "defaultSchedulingPeriodsBySchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_periods_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The name of the default scheduling strategy
    #[serde(rename = "defaultSchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_strategy: Option<String>,
    /// The default penalization period in milliseconds
    #[serde(rename = "penalizationPeriodMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalization_period_millis: Option<i64>,
    /// The default yield duration in milliseconds
    #[serde(rename = "yieldDurationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration_millis: Option<i64>,
}

/// The nearest versioned ancestor group of the component that matched the search.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultGroupDto {
    /// The id of the group.
    pub id: String,
    /// The name of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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

/// The snippet.
#[non_exhaustive]
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
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the labels in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the output ports in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The group id for the components in the snippet.
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// The ids of the process groups in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(rename = "processGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the processors in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the remote process groups in this snippet.
    /// These ids will be populated within each response.
    /// They can be specified when creating a snippet.
    /// However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(rename = "remoteProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The URI of the snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<SnippetDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartVersionControlRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionedFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow: Option<VersionedFlowDto>,
}

/// The state.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateEntryDto {
    /// The label for the node where the state originated.
    #[serde(rename = "clusterNodeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The identifier for the node where the state originated.
    #[serde(rename = "clusterNodeId")]
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
#[non_exhaustive]
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
    #[serde(rename = "totalEntryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i32>,
}

/// Indicates if the component stores state
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stateful {
    /// Description of what information is being stored in the StateManager
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates the Scope(s) associated with the State that is stored and retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// The Descriptors that provide information on each of the metrics provided in the status history
#[non_exhaustive]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryDto {
    /// A list of StatusSnapshotDTO objects that provide the actual metric values for the component. If the NiFi instance is clustered, this will represent the aggregate status across all nodes. If the NiFi instance is not clustered, this will represent the status of the entire NiFi instance.
    #[serde(rename = "aggregateSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshots: Option<Vec<StatusSnapshotDto>>,
    /// A Map of key/value pairs that describe the component that the status history belongs to
    #[serde(rename = "componentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_details: Option<std::collections::HashMap<String, Option<String>>>,
    /// The Descriptors that provide information on each of the metrics provided in the status history
    #[serde(rename = "fieldDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_descriptors: Option<Vec<StatusDescriptorDto>>,
    /// When the status history was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
    /// The NodeStatusSnapshotsDTO objects that provide the actual metric values for the component, for each node. If the NiFi instance is not clustered, this value will be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeStatusSnapshotsDto>>,
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

/// A list of StatusSnapshotDTO objects that provide the actual metric values for the component for this node.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSnapshotDto {
    /// The status metrics.
    #[serde(rename = "statusMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_metrics: Option<std::collections::HashMap<String, Option<i64>>>,
    /// The timestamp of the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// The provenance repository storage usage.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageUsageDto {
    /// Amount of free space.
    #[serde(rename = "freeSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_space: Option<String>,
    /// The number of bytes of free space.
    #[serde(rename = "freeSpaceBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_space_bytes: Option<i64>,
    /// The identifier of this storage location. The identifier will correspond to the identifier keyed in the storage configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Amount of total space.
    #[serde(rename = "totalSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_space: Option<String>,
    /// The number of bytes of total space.
    #[serde(rename = "totalSpaceBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_space_bytes: Option<i64>,
    /// Amount of used space.
    #[serde(rename = "usedSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_space: Option<String>,
    /// The number of bytes of used space.
    #[serde(rename = "usedSpaceBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_space_bytes: Option<i64>,
    /// Utilization of this storage location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingOutput {}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReplayRequestEntity {
    /// The identifier of the node where to submit the replay request.
    #[serde(rename = "clusterNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The event identifier
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
}

/// The mime types this Content Viewer supports.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedMimeTypesDto {
    /// The display name of the mime types.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The mime types this Content Viewer supports.
    #[serde(rename = "mimeTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_types: Option<Vec<String>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<SystemDiagnosticsSnapshotDto>,
    /// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeSystemDiagnosticsSnapshotDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsEntity {
    pub system_diagnostics: Option<SystemDiagnosticsDto>,
}

/// The System Diagnostics snapshot from the node.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsSnapshotDto {
    /// Number of available processors if supported by the underlying system.
    #[serde(rename = "availableProcessors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_processors: Option<i32>,
    /// The content repository storage usage.
    #[serde(rename = "contentRepositoryStorageUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    /// Number of daemon threads.
    #[serde(rename = "daemonThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_threads: Option<i32>,
    #[serde(rename = "flowFileRepositoryStorageUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_repository_storage_usage: Option<StorageUsageDto>,
    /// Amount of free heap.
    #[serde(rename = "freeHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_heap: Option<String>,
    /// The number of bytes that are allocated to the JVM heap but not currently being used
    #[serde(rename = "freeHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_heap_bytes: Option<i64>,
    /// Amount of free non heap.
    #[serde(rename = "freeNonHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_non_heap: Option<String>,
    /// Total number of free non-heap bytes available to the JVM
    #[serde(rename = "freeNonHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_non_heap_bytes: Option<i64>,
    /// The garbage collection details.
    #[serde(rename = "garbageCollection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garbage_collection: Option<Vec<GarbageCollectionDto>>,
    /// Utilization of heap.
    #[serde(rename = "heapUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heap_utilization: Option<String>,
    /// Maximum size of heap.
    #[serde(rename = "maxHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_heap: Option<String>,
    /// The maximum number of bytes that can be used by the JVM
    #[serde(rename = "maxHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_heap_bytes: Option<i64>,
    /// Maximum size of non heap.
    #[serde(rename = "maxNonHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_non_heap: Option<String>,
    /// The maximum number of bytes that the JVM can use for non-heap purposes
    #[serde(rename = "maxNonHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_non_heap_bytes: Option<i64>,
    /// Utilization of non heap.
    #[serde(rename = "nonHeapUtilization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_heap_utilization: Option<String>,
    /// The processor load average if supported by the underlying system.
    #[serde(rename = "processorLoadAverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_load_average: Option<f64>,
    /// The provenance repository storage usage.
    #[serde(rename = "provenanceRepositoryStorageUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    #[serde(rename = "resourceClaimDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_details: Option<Vec<ResourceClaimDetailsDto>>,
    /// When the diagnostics were generated.
    #[serde(rename = "statsLastRefreshed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// Total size of heap.
    #[serde(rename = "totalHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heap: Option<String>,
    /// The total number of bytes that are available for the JVM heap to use
    #[serde(rename = "totalHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heap_bytes: Option<i64>,
    /// Total size of non heap.
    #[serde(rename = "totalNonHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_non_heap: Option<String>,
    /// Total number of bytes allocated to the JVM not used for heap
    #[serde(rename = "totalNonHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_non_heap_bytes: Option<i64>,
    /// Total number of threads.
    #[serde(rename = "totalThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_threads: Option<i32>,
    /// The uptime of the Java virtual machine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
    /// Amount of used heap.
    #[serde(rename = "usedHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_heap: Option<String>,
    /// The number of bytes of JVM heap that are currently being used
    #[serde(rename = "usedHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_heap_bytes: Option<i64>,
    /// Amount of use non heap.
    #[serde(rename = "usedNonHeap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_non_heap: Option<String>,
    /// Total number of bytes used by the JVM not in the heap space
    #[serde(rename = "usedNonHeapBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_non_heap_bytes: Option<i64>,
    #[serde(rename = "versionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfoDto>,
}

/// The system resource considerations for the given component
#[non_exhaustive]
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

#[non_exhaustive]
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
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

/// The set of user group IDs associated with this access policy.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<TenantDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantsEntity {
    #[serde(rename = "userGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<TenantEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TenantEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResultEntity {
    #[serde(rename = "flowFileSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_sent: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateControllerServiceReferenceRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The identifier of the Controller Service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The revisions for all referencing components.
    #[serde(rename = "referencingComponentRevisions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_component_revisions:
        Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The new state of the references for the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Indicates whether or not the response should only include fields necessary for rendering the NiFi User Interface.
    /// As such, when this value is set to true, some fields may be returned as null values, and the selected fields may change at any time without notice.
    /// As a result, this value should not be set to true by any client other than the UI.
    #[serde(rename = "uiOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_only: Option<bool>,
}

/// A list of use cases that have been documented for this Processor
#[non_exhaustive]
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
    #[serde(rename = "inputRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<String>,
    /// Keywords that pertain to the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Any pertinent notes about the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    /// The access policies this user belongs to.
    #[serde(rename = "accessPolicies")]
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
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The groups to which the user belongs. This field is read only and it provided for convenience.
    #[serde(rename = "userGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<UserDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupDto {
    /// The access policies this user group belongs to. This field was incorrectly defined as an AccessPolicyEntity. For compatibility reasons the field will remain of this type, however only the fields that are present in the AccessPolicySummaryEntity will be populated here.
    #[serde(rename = "accessPolicies")]
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
    #[serde(rename = "parentGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The users that belong to the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    #[serde(rename = "versionedComponentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<UserGroupDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
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

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupsEntity {
    #[serde(rename = "userGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<UserGroupEntity>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersEntity {
    /// When this content was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserEntity>>,
}

/// The request
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigRequestDto {
    /// FlowFile Attributes that should be used to evaluate Expression Language for resolving property values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether or not the request is completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The ID of the component whose configuration was verified
    #[serde(rename = "componentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The reason for the request failing, or null if the request has not failed
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The configured component properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The ID of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The Results of the verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConfigVerificationResultDto>>,
    /// A description of the current state of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted
    #[serde(rename = "submissionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each
    #[serde(rename = "updateSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<VerifyConfigUpdateStepDto>>,
    /// The URI for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigRequestEntity {
    pub request: Option<VerifyConfigRequestDto>,
}

/// The steps that are required in order to complete the request, along with the status of each
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigUpdateStepDto {
    /// Whether or not this step has completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlComponentMappingEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    /// The mapping of Versioned Component Identifiers to instance ID's
    #[serde(rename = "versionControlComponentMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_component_mapping:
        Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "versionControlInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
}

/// The Version Control information
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlInformationDto {
    /// The ID of the branch that the flow is stored in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The ID of the bucket that the flow is stored in
    #[serde(rename = "bucketId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// The name of the bucket that the flow is stored in
    #[serde(rename = "bucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The description of the flow
    #[serde(rename = "flowDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_description: Option<String>,
    /// The ID of the flow
    #[serde(rename = "flowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The name of the flow
    #[serde(rename = "flowName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    /// The ID of the Process Group that is under version control
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The ID of the registry that the flow is stored in
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// The name of the registry that the flow is stored in
    #[serde(rename = "registryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// The current state of the Process Group, as it relates to the Versioned Flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Explanation of why the group is in the specified state
    #[serde(rename = "stateExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_explanation: Option<String>,
    /// The storage location
    #[serde(rename = "storageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    /// The version of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlInformationEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionControlInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
}

/// The nifi, os, java, and build version information
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfoDto {
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
    /// Java JVM vendor
    #[serde(rename = "javaVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_vendor: Option<String>,
    /// Java version
    #[serde(rename = "javaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<String>,
    /// The version of this NiFi.
    #[serde(rename = "niFiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ni_fi_version: Option<String>,
    /// Host operating system architecture
    #[serde(rename = "osArchitecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<String>,
    /// Host operating system name
    #[serde(rename = "osName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Host operating system version
    #[serde(rename = "osVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
}

/// The assets that are referenced by this parameter
#[non_exhaustive]
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

/// The Connections
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedConnection {
    /// The object data size threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    #[serde(rename = "backPressureDataSizeThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_data_size_threshold: Option<String>,
    /// The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    #[serde(rename = "backPressureObjectThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_object_threshold: Option<i64>,
    /// The bend points on the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bends: Option<Vec<Position>>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ConnectableComponent>,
    /// The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it.
    #[serde(rename = "flowFileExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_expiration: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The index of the bend point where to place the connection label.
    #[serde(rename = "labelIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_index: Option<i32>,
    /// Whether or not compression should be used when transferring FlowFiles between nodes Possible returned values: DO_NOT_COMPRESS, COMPRESS_ATTRIBUTES_ONLY, COMPRESS_ATTRIBUTES_AND_CONTENT. See LoadBalanceCompression.class for more details.
    #[serde(rename = "loadBalanceCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_compression: Option<String>,
    /// The Strategy to use for load balancing data across the cluster, or null, if no Load Balance Strategy has been specified. Possible returned values: DO_NOT_LOAD_BALANCE, PARTITION_BY_ATTRIBUTE, ROUND_ROBIN, SINGLE_NODE. See LoadBalanceStrategy.class for more details.
    #[serde(rename = "loadBalanceStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_strategy: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The attribute to use for partitioning data as it is load balanced across the cluster. If the Load Balance Strategy is configured to use PARTITION_BY_ATTRIBUTE, the value returned by this method is the name of the FlowFile Attribute that will be used to determine which node in the cluster should receive a given FlowFile. If the Load Balance Strategy is unset or is set to any other value, the Partitioning Attribute has no effect.
    #[serde(rename = "partitioningAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitioning_attribute: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The comparators used to prioritize the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritizers: Option<Vec<String>>,
    /// The selected relationship that comprise the connection.
    #[serde(rename = "selectedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_relationships: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ConnectableComponent>,
    /// The z index of the connection.
    #[serde(rename = "zIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i64>,
}

/// The Controller Services
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedControllerService {
    /// The annotation for the controller service. This is how the custom UI relays configuration to the controller service.
    #[serde(rename = "annotationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The level at which the controller service will report bulletins.
    #[serde(rename = "bulletinLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// Lists the APIs this Controller Service implements.
    #[serde(rename = "controllerServiceApis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_apis: Option<Vec<ControllerServiceAPI>>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
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
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// The ScheduledState denoting whether the Controller Service is ENABLED or DISABLED
    #[serde(rename = "scheduledState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<String>,
    /// The type of the extension component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// The coordinates where the remote flow is stored, or null if the Process Group is not directly under Version Control
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowCoordinates {
    /// The name of the branch that the flow resides in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The UUID of the bucket that the flow resides in
    #[serde(rename = "bucketId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// The UUID of the flow
    #[serde(rename = "flowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// Whether or not these coordinates point to the latest version of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    /// The identifier of the Flow Registry that contains the flow
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// The location of the Flow Registry that stores the flow
    #[serde(rename = "storageLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    /// The version of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
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
pub struct VersionedFlowSnapshotEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    /// The ID of the Registry that this flow belongs to
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// If the Process Group to be updated has a child or descendant Process Group that is also under Version Control, this specifies whether or not the contents of that child/descendant Process Group should be updated.
    #[serde(rename = "updateDescendantVersionedFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_descendant_versioned_flows: Option<bool>,
    #[serde(rename = "versionedFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow: Option<RegisteredFlowSnapshot>,
    #[serde(rename = "versionedFlowSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataEntity {
    /// The ID of the Registry that this flow belongs to
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "versionedFlowSnapshotMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataSetEntity {
    #[serde(rename = "versionedFlowSnapshotMetadataSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot_metadata_set: Option<Vec<VersionedFlowSnapshotMetadataEntity>>,
}

/// The Flow Update Request
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestDto {
    /// Whether or not this request has completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// An explanation of why this request failed, or null if this request has not failed
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The last time this request was updated.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The percentage complete for the request, between 0 and 100
    #[serde(rename = "percentCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The unique ID of the Process Group being updated
    #[serde(rename = "processGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The unique ID of this request.
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The state of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The URI for future requests to this drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "versionControlInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestEntity {
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<VersionedFlowUpdateRequestDto>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowsEntity {
    #[serde(rename = "versionedFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flows: Option<Vec<VersionedFlowEntity>>,
}

/// The Funnels
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFunnel {
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
}

/// The Labels
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedLabel {
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The height of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
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
    #[serde(rename = "zIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i64>,
}

/// Returns the Listen Port Definition for the port this property specifies, if applicable
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedListenPortDefinition {
    /// The application protocol(s) that the listen port could support (if any)
    #[serde(rename = "applicationProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocols: Option<Vec<String>>,
    /// The transport protocol used by the listen port
    #[serde(rename = "transportProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
}

/// The parameters in the context
#[non_exhaustive]
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
    #[serde(rename = "referencedAssets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_assets: Option<Vec<VersionedAsset>>,
    /// Whether or not the parameter value is sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// The value of the parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedParameterContext {
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The description of the parameter context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The names of additional parameter contexts from which to inherit parameters
    #[serde(rename = "inheritedParameterContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_parameter_contexts: Option<Vec<String>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The corresponding parameter group name fetched from the parameter provider, if applicable
    #[serde(rename = "parameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// The identifier of an optional parameter provider
    #[serde(rename = "parameterProvider")]
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

/// The Output Ports
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedPort {
    /// Whether or not this port allows remote access for site-to-site
    #[serde(rename = "allowRemoteAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_access: Option<bool>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The number of tasks that should be concurrently scheduled for the port.
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specifies how the Port should function
    #[serde(rename = "portFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The scheduled state of the component
    #[serde(rename = "scheduledState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<String>,
    /// The type of port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedProcessGroup {
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The Connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<VersionedConnection>>,
    /// The Controller Services
    #[serde(rename = "controllerServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<VersionedControllerService>>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureDataSizeThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    #[serde(rename = "defaultBackPressureObjectThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// The default FlowFile Expiration for this Process Group.
    #[serde(rename = "defaultFlowFileExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_flow_file_expiration: Option<String>,
    /// The Execution Engine that should be used to run the components within the group.
    #[serde(rename = "executionEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_engine: Option<String>,
    /// The configured FlowFile Concurrency for the Process Group
    #[serde(rename = "flowFileConcurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_concurrency: Option<String>,
    /// The FlowFile Outbound Policy for the Process Group
    #[serde(rename = "flowFileOutboundPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_outbound_policy: Option<String>,
    /// The Funnels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<VersionedFunnel>>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The Input Ports
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<VersionedPort>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The Labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<VersionedLabel>>,
    /// The log file suffix for this Process Group for dedicated logging.
    #[serde(rename = "logFileSuffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_suffix: Option<String>,
    /// The maximum number of concurrent tasks that should be scheduled for this Process Group when using the Stateless Engine
    #[serde(rename = "maxConcurrentTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_tasks: Option<i32>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Output Ports
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<VersionedPort>>,
    /// The name of the parameter context used by this process group
    #[serde(rename = "parameterContextName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The child Process Groups
    #[serde(rename = "processGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<Box<VersionedProcessGroup>>>,
    /// The Processors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<VersionedProcessor>>,
    /// The Remote Process Groups
    #[serde(rename = "remoteProcessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
    /// The Scheduled State of the Process Group, if the group is configured to use the Stateless Execution Engine. Otherwise, this value has no relevance.
    #[serde(rename = "scheduledState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<String>,
    /// The maximum amount of time that the flow is allows to run using the Stateless engine before it times out and is considered a failure
    #[serde(rename = "statelessFlowTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_flow_timeout: Option<String>,
    #[serde(rename = "versionedFlowCoordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_coordinates: Option<VersionedFlowCoordinates>,
}

/// The Processors
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedProcessor {
    /// The annotation data for the processor used to relay configuration between a custom UI and the procesosr.
    #[serde(rename = "annotationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated.
    #[serde(rename = "autoTerminatedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminated_relationships: Option<Vec<String>>,
    /// Determines whether the FlowFile should be penalized or the processor should be yielded between retries. Possible returned values: PENALIZE_FLOWFILE, YIELD_PROCESSOR.
    #[serde(rename = "backoffMechanism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_mechanism: Option<String>,
    /// The level at which the processor will report bulletins.
    #[serde(rename = "bulletinLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored.
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// Indicates the node where the process will execute.
    #[serde(rename = "executionNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// Maximum amount of time to be waited during a retry period.
    #[serde(rename = "maxBackoffPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_backoff_period: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The amout of time that is used when the process penalizes a flowfile.
    #[serde(rename = "penaltyDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The property descriptors for the component.
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// All the relationships should be retried.
    #[serde(rename = "retriedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_relationships: Option<Vec<String>>,
    /// Overall number of retries.
    #[serde(rename = "retryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    /// The run duration for the processor in milliseconds.
    #[serde(rename = "runDurationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_duration_millis: Option<i64>,
    /// The scheduled state of the component
    #[serde(rename = "scheduledState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<String>,
    /// The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy.
    #[serde(rename = "schedulingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// Indicates how the processor should be scheduled to run.
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// Stylistic data for rendering in a UI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The type of the extension component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The amount of time that must elapse before this processor is scheduled again after yielding.
    #[serde(rename = "yieldDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
}

/// The property descriptors for the component.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedPropertyDescriptor {
    /// The display name of the property
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether or not the property is user-defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    /// Whether or not the property provides the identifier of a Controller Service
    #[serde(rename = "identifiesControllerService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifies_controller_service: Option<bool>,
    #[serde(rename = "listenPortDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port_definition: Option<VersionedListenPortDefinition>,
    /// The name of the property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition: Option<VersionedResourceDefinition>,
    /// Whether or not the property is considered sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
}

/// A Set of Output Ports that can be connected to, in order to pull data from the remote NiFi instance
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedRemoteGroupPort {
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<BatchSize>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The number of task that may transmit flowfiles to the target port concurrently.
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The id of the remote process group that the port resides in.
    #[serde(rename = "remoteGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_group_id: Option<String>,
    /// The scheduled state of the component
    #[serde(rename = "scheduledState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<String>,
    /// The ID of the port on the target NiFi instance
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// Whether the flowfiles are compressed when sent to the target port.
    #[serde(rename = "useCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_compression: Option<bool>,
}

/// The Remote Process Groups
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedRemoteProcessGroup {
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The time period used for the timeout when communicating with the target.
    #[serde(rename = "communicationsTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications_timeout: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// A Set of Input Ports that can be connected to, in order to send data to the remote NiFi instance
    #[serde(rename = "inputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<VersionedRemoteGroupPort>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier.
    #[serde(rename = "localNetworkInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_network_interface: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A Set of Output Ports that can be connected to, in order to pull data from the remote NiFi instance
    #[serde(rename = "outputPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<VersionedRemoteGroupPort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(rename = "proxyHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_host: Option<String>,
    #[serde(rename = "proxyPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,
    #[serde(rename = "proxyPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i32>,
    #[serde(rename = "proxyUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_user: Option<String>,
    /// The target URIs of the remote process group. If target uris is not set but target uri is set, then returns the single target uri. If neither target uris nor target uri is set, then returns null.
    #[serde(rename = "targetUris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uris: Option<String>,
    /// The Transport Protocol that is used for Site-to-Site communications. Possible returned values: RAW, HTTP.
    #[serde(rename = "transportProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
    /// When yielding, this amount of time must elapse before the remote process group is scheduled again.
    #[serde(rename = "yieldDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
}

/// The reporting tasks
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTask {
    /// The annotation for the reporting task. This is how the custom UI relays configuration to the reporting task.
    #[serde(rename = "annotationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(rename = "groupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(rename = "instanceIdentifier")]
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
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// Indicates the scheduled state for the Reporting Task
    #[serde(rename = "scheduledState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<String>,
    /// The frequency with which to schedule the reporting task. The format of the value will depend on the value of schedulingStrategy.
    #[serde(rename = "schedulingPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// Indicates scheduling strategy that should dictate how the reporting task is triggered.
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// The type of the extension component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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

/// Returns the Resource Definition that defines which type(s) of resource(s) this property references, if any
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedResourceDefinition {
    /// The cardinality of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<String>,
    /// The types of resource that the Property Descriptor is allowed to reference
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}
