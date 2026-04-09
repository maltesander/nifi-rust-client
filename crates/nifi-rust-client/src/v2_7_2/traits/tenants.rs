// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The Tenants API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait TenantsApi {
    /// Searches for a tenant with the specified identity
    async fn search_tenants(
        &self,
        q: &str,
    ) -> Result<crate::v2_7_2::types::TenantsEntity, NifiError>;
    /// Gets all user groups
    async fn get_user_groups(&self) -> Result<crate::v2_7_2::types::UserGroupsEntity, NifiError>;
    /// Creates a user group
    async fn create_user_group(
        &self,
        body: &crate::v2_7_2::types::UserGroupEntity,
    ) -> Result<crate::v2_7_2::types::UserGroupEntity, NifiError>;
    /// Deletes a user group
    async fn remove_user_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::UserGroupEntity, NifiError>;
    /// Gets a user group
    async fn get_user_group(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::UserGroupEntity, NifiError>;
    /// Updates a user group
    async fn update_user_group(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::UserGroupEntity,
    ) -> Result<crate::v2_7_2::types::UserGroupEntity, NifiError>;
    /// Gets all users
    async fn get_users(&self) -> Result<crate::v2_7_2::types::UsersEntity, NifiError>;
    /// Creates a user
    async fn create_user(
        &self,
        body: &crate::v2_7_2::types::UserEntity,
    ) -> Result<crate::v2_7_2::types::UserEntity, NifiError>;
    /// Deletes a user
    async fn remove_user(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::UserEntity, NifiError>;
    /// Gets a user
    async fn get_user(&self, id: &str) -> Result<crate::v2_7_2::types::UserEntity, NifiError>;
    /// Updates a user
    async fn update_user(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::UserEntity,
    ) -> Result<crate::v2_7_2::types::UserEntity, NifiError>;
}
