// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ConnectionsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ConnectionsApi<'a> {
    /// Deletes a connection
    ///
    /// Calls `DELETE /nifi-api/connections/{id}`.
    ///
    /// # Parameters
    /// - `id`: The connection id.
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
    /// - `Write Source - /{component-type}/{uuid}`
    /// - `Write - Parent Process Group - /process-groups/{uuid}`
    /// - `Write Destination - /{component-type}/{uuid}`
    pub async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ConnectionEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/connections/{id}"), &query)
            .await
    }
    /// Gets a connection
    ///
    /// Calls `GET /nifi-api/connections/{id}`.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read Source - /{component-type}/{uuid}`
    /// - `Read Destination - /{component-type}/{uuid}`
    pub async fn get_connection(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::ConnectionEntity, NifiError> {
        self.client.get(&format!("/connections/{id}")).await
    }
    /// Updates a connection
    ///
    /// Calls `PUT /nifi-api/connections/{id}`.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    /// - `body`: The connection configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Write Source - /{component-type}/{uuid}`
    /// - `Write Destination - /{component-type}/{uuid}`
    /// - `Write New Destination - /{component-type}/{uuid} - if updating Destination`
    /// - `Write Process Group - /process-groups/{uuid} - if updating Destination`
    pub async fn update_connection(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::ConnectionEntity,
    ) -> Result<crate::v2_8_0::types::ConnectionEntity, NifiError> {
        self.client.put(&format!("/connections/{id}"), body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::ConnectionsApi for ConnectionsApi<'_> {
    async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ConnectionEntity, NifiError> {
        self.delete_connection(id, version, client_id, disconnected_node_acknowledged)
            .await
    }
    async fn get_connection(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::ConnectionEntity, NifiError> {
        self.get_connection(id).await
    }
    async fn update_connection(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::ConnectionEntity,
    ) -> Result<crate::v2_8_0::types::ConnectionEntity, NifiError> {
        self.update_connection(id, body).await
    }
}
