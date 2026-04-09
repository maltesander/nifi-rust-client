// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::TenantsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Dynamic dispatch enum for the Tenants API. Use via the [`TenantsApi`] trait.
#[allow(private_interfaces)]
#[non_exhaustive]
pub enum TenantsApiDispatch<'a> {
    V2_6_0(super::super::impls::v2_6_0::V2_6_0TenantsApi<'a>),
    V2_7_2(super::super::impls::v2_7_2::V2_7_2TenantsApi<'a>),
    V2_8_0(super::super::impls::v2_8_0::V2_8_0TenantsApi<'a>),
}
impl<'a> TenantsApiDispatch<'a> {
    fn client(&self) -> &'a crate::NifiClient {
        match self {
            Self::V2_6_0(api) => api.client,
            Self::V2_7_2(api) => api.client,
            Self::V2_8_0(api) => api.client,
        }
    }
    fn version(&self) -> crate::dynamic::DetectedVersion {
        match self {
            Self::V2_6_0(_) => crate::dynamic::DetectedVersion::V2_6_0,
            Self::V2_7_2(_) => crate::dynamic::DetectedVersion::V2_7_2,
            Self::V2_8_0(_) => crate::dynamic::DetectedVersion::V2_8_0,
        }
    }
}
impl TenantsApi for TenantsApiDispatch<'_> {
    async fn create_user(&self, body: &types::UserEntity) -> Result<types::UserEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_user(body).await,
            Self::V2_7_2(api) => api.create_user(body).await,
            Self::V2_8_0(api) => api.create_user(body).await,
        }
    }
    async fn create_user_group(
        &self,
        body: &types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.create_user_group(body).await,
            Self::V2_7_2(api) => api.create_user_group(body).await,
            Self::V2_8_0(api) => api.create_user_group(body).await,
        }
    }
    async fn get_user(&self, id: &str) -> Result<types::UserEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_user(id).await,
            Self::V2_7_2(api) => api.get_user(id).await,
            Self::V2_8_0(api) => api.get_user(id).await,
        }
    }
    async fn get_user_group(&self, id: &str) -> Result<types::UserGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_user_group(id).await,
            Self::V2_7_2(api) => api.get_user_group(id).await,
            Self::V2_8_0(api) => api.get_user_group(id).await,
        }
    }
    async fn get_user_groups(&self) -> Result<types::UserGroupsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_user_groups().await,
            Self::V2_7_2(api) => api.get_user_groups().await,
            Self::V2_8_0(api) => api.get_user_groups().await,
        }
    }
    async fn get_users(&self) -> Result<types::UsersEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.get_users().await,
            Self::V2_7_2(api) => api.get_users().await,
            Self::V2_8_0(api) => api.get_users().await,
        }
    }
    async fn remove_user(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_user(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_user(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_user(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn remove_user_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => {
                api.remove_user_group(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_7_2(api) => {
                api.remove_user_group(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
            Self::V2_8_0(api) => {
                api.remove_user_group(id, version, client_id, disconnected_node_acknowledged)
                    .await
            }
        }
    }
    async fn search_tenants(&self, q: &str) -> Result<types::TenantsEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.search_tenants(q).await,
            Self::V2_7_2(api) => api.search_tenants(q).await,
            Self::V2_8_0(api) => api.search_tenants(q).await,
        }
    }
    async fn update_user(
        &self,
        id: &str,
        body: &types::UserEntity,
    ) -> Result<types::UserEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_user(id, body).await,
            Self::V2_7_2(api) => api.update_user(id, body).await,
            Self::V2_8_0(api) => api.update_user(id, body).await,
        }
    }
    async fn update_user_group(
        &self,
        id: &str,
        body: &types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        match self {
            Self::V2_6_0(api) => api.update_user_group(id, body).await,
            Self::V2_7_2(api) => api.update_user_group(id, body).await,
            Self::V2_8_0(api) => api.update_user_group(id, body).await,
        }
    }
}
