// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct FunnelsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> FunnelsApi<'a> {
    /// Deletes a funnel
    ///
    /// Calls `DELETE /nifi-api/funnels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The funnel id.
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
    /// - `Write - /funnels/{uuid}`
    /// - `Write - Parent Process Group - /process-groups/{uuid}`
    pub async fn remove_funnel(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::FunnelEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/funnels/{id}"), &query)
            .await
    }
    /// Gets a funnel
    ///
    /// Calls `GET /nifi-api/funnels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The funnel id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /funnels/{uuid}`.
    pub async fn get_funnel(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::FunnelEntity, NifiError> {
        self.client.get(&format!("/funnels/{id}")).await
    }
    /// Updates a funnel
    ///
    /// Calls `PUT /nifi-api/funnels/{id}`.
    ///
    /// # Parameters
    /// - `id`: The funnel id.
    /// - `body`: The funnel configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /funnels/{uuid}`.
    pub async fn update_funnel(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::FunnelEntity,
    ) -> Result<crate::v2_7_2::types::FunnelEntity, NifiError> {
        self.client.put(&format!("/funnels/{id}"), body).await
    }
}
