#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantsEntity {
    pub user_groups: Option<Vec<TenantEntity>>,
    pub users: Option<Vec<TenantEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<UserDto>,
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
pub struct UserGroupEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<UserGroupDto>,
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
pub struct UserGroupsEntity {
    pub user_groups: Option<Vec<UserGroupEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersEntity {
    /// When this content was generated.
    pub generated: Option<String>,
    pub users: Option<Vec<UserEntity>>,
}
