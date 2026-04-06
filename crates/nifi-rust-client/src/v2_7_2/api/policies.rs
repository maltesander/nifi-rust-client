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
    pub async fn create_access_policy(
        &self,
        body: &crate::types::AccessPolicyEntity,
    ) -> Result<crate::types::AccessPolicyEntity, NifiError> {
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
    pub async fn get_access_policy_for_resource(
        &self,
        action: &str,
        resource: &str,
    ) -> Result<crate::types::AccessPolicyEntity, NifiError> {
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
    pub async fn remove_access_policy(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::types::AccessPolicyEntity, NifiError> {
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
    pub async fn get_access_policy(
        &self,
        id: &str,
    ) -> Result<crate::types::AccessPolicyEntity, NifiError> {
        self.client.get(&format!("/policies/{id}")).await
    }
    /// Updates a access policy
    ///
    /// Calls `PUT /nifi-api/policies/{id}`.
    ///
    /// # Parameters
    /// - `id`: The access policy id.
    /// - `body`: The access policy configuration details.
    pub async fn update_access_policy(
        &self,
        id: &str,
        body: &crate::types::AccessPolicyEntity,
    ) -> Result<crate::types::AccessPolicyEntity, NifiError> {
        self.client.put(&format!("/policies/{id}"), body).await
    }
}
