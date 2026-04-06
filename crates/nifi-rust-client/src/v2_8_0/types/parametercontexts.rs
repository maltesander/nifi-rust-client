#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// The Asset.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDto {
    /// The digest of the asset, will be null if the asset content is missing.
    pub digest: Option<String>,
    /// The identifier of the asset.
    pub id: Option<String>,
    /// Indicates if the content of the asset is missing.
    pub missing_content: Option<bool>,
    /// The name of the asset.
    pub name: Option<String>,
}
/// The asset entities
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetEntity {
    pub asset: AssetDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsEntity {
    /// The asset entities
    pub assets: Option<Vec<AssetEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ParameterContextDto>,
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
pub struct ParameterContextUpdateRequestEntity {
    pub parameter_context_revision: Option<RevisionDto>,
    pub request: Option<ParameterContextUpdateRequestDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub request: Option<ParameterContextValidationRequestDto>,
}
