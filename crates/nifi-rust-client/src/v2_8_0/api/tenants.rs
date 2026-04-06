// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct TenantsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> TenantsApi<'a> {
    /// Searches for a tenant with the specified identity
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/search-results`.
    ///
    /// # Parameters
    /// - `q`: Identity to search for.
    pub async fn search_tenants(
        &self,
        q: &str,
    ) -> Result<crate::v2_8_0::types::TenantsEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("q", q.to_string()));
        self.client
            .get_with_query("/tenants/search-results", &query)
            .await
    }
    /// Gets all user groups
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/user-groups`.
    pub async fn get_user_groups(
        &self,
    ) -> Result<crate::v2_8_0::types::UserGroupsEntity, NifiError> {
        self.client.get("/tenants/user-groups").await
    }
    /// Creates a user group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/tenants/user-groups`.
    ///
    /// # Parameters
    /// - `body`: The user group configuration details.
    pub async fn create_user_group(
        &self,
        body: &crate::v2_8_0::types::UserGroupEntity,
    ) -> Result<crate::v2_8_0::types::UserGroupEntity, NifiError> {
        self.client.post("/tenants/user-groups", body).await
    }
    /// Deletes a user group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/tenants/user-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user group id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_user_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::UserGroupEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = version {
            query.push(("version", v.to_string()));
        }
        if let Some(v) = client_id {
            query.push(("clientId", v.to_string()));
        }
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/tenants/user-groups/{id}"), &query)
            .await
    }
    /// Gets a user group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/user-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user group id.
    pub async fn get_user_group(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::UserGroupEntity, NifiError> {
        self.client.get(&format!("/tenants/user-groups/{id}")).await
    }
    /// Updates a user group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/tenants/user-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user group id.
    /// - `body`: The user group configuration details.
    pub async fn update_user_group(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::UserGroupEntity,
    ) -> Result<crate::v2_8_0::types::UserGroupEntity, NifiError> {
        self.client
            .put(&format!("/tenants/user-groups/{id}"), body)
            .await
    }
    /// Gets all users
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/users`.
    pub async fn get_users(&self) -> Result<crate::v2_8_0::types::UsersEntity, NifiError> {
        self.client.get("/tenants/users").await
    }
    /// Creates a user
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/tenants/users`.
    ///
    /// # Parameters
    /// - `body`: The user configuration details.
    pub async fn create_user(
        &self,
        body: &crate::v2_8_0::types::UserEntity,
    ) -> Result<crate::v2_8_0::types::UserEntity, NifiError> {
        self.client.post("/tenants/users", body).await
    }
    /// Deletes a user
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/tenants/users/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_user(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::UserEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = version {
            query.push(("version", v.to_string()));
        }
        if let Some(v) = client_id {
            query.push(("clientId", v.to_string()));
        }
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/tenants/users/{id}"), &query)
            .await
    }
    /// Gets a user
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/users/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user id.
    pub async fn get_user(&self, id: &str) -> Result<crate::v2_8_0::types::UserEntity, NifiError> {
        self.client.get(&format!("/tenants/users/{id}")).await
    }
    /// Updates a user
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/tenants/users/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user id.
    /// - `body`: The user configuration details.
    pub async fn update_user(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::UserEntity,
    ) -> Result<crate::v2_8_0::types::UserEntity, NifiError> {
        self.client.put(&format!("/tenants/users/{id}"), body).await
    }
}
