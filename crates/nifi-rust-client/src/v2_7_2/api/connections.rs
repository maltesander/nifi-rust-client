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
    pub async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::ConnectionEntity, NifiError> {
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
    pub async fn get_connection(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::ConnectionEntity, NifiError> {
        self.client.get(&format!("/connections/{id}")).await
    }
    /// Updates a connection
    ///
    /// Calls `PUT /nifi-api/connections/{id}`.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    /// - `body`: The connection configuration details.
    pub async fn update_connection(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::ConnectionEntity,
    ) -> Result<crate::v2_7_2::types::ConnectionEntity, NifiError> {
        self.client.put(&format!("/connections/{id}"), body).await
    }
}
