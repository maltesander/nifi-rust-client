// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Tenants API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait TenantsApi {
    /// Creates a user
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/tenants/users`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /tenants`.
    async fn create_user(&self, body: &types::UserEntity) -> Result<types::UserEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_user".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a user group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/tenants/user-groups`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /tenants`.
    async fn create_user_group(
        &self,
        body: &types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_user_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a user
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/users/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /tenants`.
    async fn get_user(&self, id: &str) -> Result<types::UserEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_user".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a user group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/user-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /tenants`.
    async fn get_user_group(&self, id: &str) -> Result<types::UserGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_user_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all user groups
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/user-groups`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /tenants`.
    async fn get_user_groups(&self) -> Result<types::UserGroupsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_user_groups".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all users
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/users`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /tenants`.
    async fn get_users(&self) -> Result<types::UsersEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_users".to_string(),
            version: "unknown".to_string(),
        })
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
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /tenants`.
    async fn remove_user(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_user".to_string(),
            version: "unknown".to_string(),
        })
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
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /tenants`.
    async fn remove_user_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::UserGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_user_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Searches for a tenant with the specified identity
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/tenants/search-results`.
    ///
    /// # Parameters
    /// - `q`: Identity to search for.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /tenants`.
    async fn search_tenants(&self, q: &str) -> Result<types::TenantsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "search_tenants".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a user
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/tenants/users/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /tenants`.
    async fn update_user(
        &self,
        id: &str,
        body: &types::UserEntity,
    ) -> Result<types::UserEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_user".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a user group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/tenants/user-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The user group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /tenants`.
    async fn update_user_group(
        &self,
        id: &str,
        body: &types::UserGroupEntity,
    ) -> Result<types::UserGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_user_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
