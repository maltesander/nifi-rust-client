// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Policies API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait PoliciesApi {
    /// Creates an access policy
    ///
    /// Calls `POST /nifi-api/policies`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /policies/{resource}`.
    async fn create_access_policy(
        &self,
        body: types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_access_policy".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets an access policy
    ///
    /// Calls `GET /nifi-api/policies/{id}`.
    ///
    /// # Parameters
    /// - `id`: The access policy id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /policies/{resource}`.
    async fn get_access_policy(&self, id: &str) -> Result<types::AccessPolicyEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_access_policy".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets an access policy for the specified action and resource
    ///
    /// Will return the effective policy if no component specific policy exists for the specified action and resource. Must have Read permissions to the policy with the desired action and resource. Permissions for the policy that is returned will be indicated in the response. This means the client could be authorized to get the policy for a given component but the effective policy may be inherited from an ancestor Process Group. If the client does not have permissions to that policy, the response will not include the policy and the permissions in the response will be marked accordingly. If the client does not have permissions to the policy of the desired action and resource a 403 response will be returned.
    ///
    /// Calls `GET /nifi-api/policies/{action}/{resource}`.
    ///
    /// # Parameters
    /// - `action`: The request action.
    /// - `resource`: The resource of the policy.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /policies/{resource}`.
    async fn get_access_policy_for_resource(
        &self,
        action: &str,
        resource: &str,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_access_policy_for_resource".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes an access policy
    ///
    /// Calls `DELETE /nifi-api/policies/{id}`.
    ///
    /// # Parameters
    /// - `id`: The access policy id.
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
    /// Requires `Write - /policies/{resource}`.
    /// Requires `Write - Policy of the parent resource - /policies/{resource}`.
    async fn remove_access_policy(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_access_policy".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a access policy
    ///
    /// Calls `PUT /nifi-api/policies/{id}`.
    ///
    /// # Parameters
    /// - `id`: The access policy id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /policies/{resource}`.
    async fn update_access_policy(
        &self,
        id: &str,
        body: types::AccessPolicyEntity,
    ) -> Result<types::AccessPolicyEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_access_policy".to_string(),
            version: "unknown".to_string(),
        })
    }
}
