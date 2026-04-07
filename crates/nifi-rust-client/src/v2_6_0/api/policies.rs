// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct PoliciesApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> PoliciesApi<'a> {
    /// Creates an access policy
    ///
    /// Calls `POST /nifi-api/policies`.
    ///
    /// # Parameters
    /// - `body`: The access policy configuration details.
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
    pub async fn create_access_policy(
        &self,
        body: &crate::v2_6_0::types::AccessPolicyEntity,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError> {
        self.client.post("/policies", body).await
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
    pub async fn get_access_policy_for_resource(
        &self,
        action: &str,
        resource: &str,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError> {
        self.client
            .get(&format!("/policies/{action}/{resource}"))
            .await
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
    /// - `Write - /policies/{resource}`
    /// - `Write - Policy of the parent resource - /policies/{resource}`
    pub async fn remove_access_policy(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/policies/{id}"), &query)
            .await
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
    pub async fn get_access_policy(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError> {
        self.client.get(&format!("/policies/{id}")).await
    }
    /// Updates a access policy
    ///
    /// Calls `PUT /nifi-api/policies/{id}`.
    ///
    /// # Parameters
    /// - `id`: The access policy id.
    /// - `body`: The access policy configuration details.
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
    pub async fn update_access_policy(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::AccessPolicyEntity,
    ) -> Result<crate::v2_6_0::types::AccessPolicyEntity, NifiError> {
        self.client.put(&format!("/policies/{id}"), body).await
    }
}
