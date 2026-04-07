// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// The Asset.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_content: Option<bool>,
    /// The name of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The asset entities
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<AssetDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsEntity {
    /// The asset entities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterContextDto>,
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
pub struct ParameterContextUpdateRequestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ParameterContextUpdateRequestDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ParameterContextValidationRequestDto>,
}
