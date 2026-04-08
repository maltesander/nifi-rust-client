// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
use crate::dynamic::traits::TenantsApi;
#[allow(unused_imports)]
use crate::dynamic::types;
pub(crate) struct V2_8_0TenantsApi<'a> {
    pub(crate) client: &'a crate::NifiClient,
}
#[allow(unused_variables)]
impl TenantsApi for V2_8_0TenantsApi<'_> {
    async fn create_user(&self, body: types::UserEntity) -> Result<types::UserEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api
            .create_user(&crate::v2_8_0::types::UserEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn create_user_group(
        &self,
        body: types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api
            .create_user_group(&crate::v2_8_0::types::UserGroupEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn get_user(&self, id: &str) -> Result<types::UserEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api.get_user(id).await?.into())
    }
    async fn get_user_group(&self, id: &str) -> Result<types::UserGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api.get_user_group(id).await?.into())
    }
    async fn get_user_groups(&self) -> Result<types::UserGroupsEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api.get_user_groups().await?.into())
    }
    async fn get_users(&self) -> Result<types::UsersEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api.get_users().await?.into())
    }
    async fn remove_user(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api
            .remove_user(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn remove_user_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api
            .remove_user_group(id, version, client_id, disconnected_node_acknowledged)
            .await?
            .into())
    }
    async fn search_tenants(&self, q: &str) -> Result<types::TenantsEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api.search_tenants(q).await?.into())
    }
    async fn update_user(
        &self,
        id: &str,
        body: types::UserEntity,
    ) -> Result<types::UserEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api
            .update_user(id, &crate::v2_8_0::types::UserEntity::try_from(body)?)
            .await?
            .into())
    }
    async fn update_user_group(
        &self,
        id: &str,
        body: types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        let api = crate::v2_8_0::api::tenants::TenantsApi {
            client: self.client,
        };
        Ok(api
            .update_user_group(id, &crate::v2_8_0::types::UserGroupEntity::try_from(body)?)
            .await?
            .into())
    }
}
