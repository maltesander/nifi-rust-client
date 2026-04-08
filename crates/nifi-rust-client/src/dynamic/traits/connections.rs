// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Connections API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ConnectionsApi {
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
    /// Requires `Write Source - /{component-type}/{uuid}`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    /// Requires `Write Destination - /{component-type}/{uuid}`.
    async fn delete_connection(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ConnectionEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_connection".to_string(),
            version: "unknown".to_string(),
        })
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
    /// Requires `Read Source - /{component-type}/{uuid}`.
    /// Requires `Read Destination - /{component-type}/{uuid}`.
    async fn get_connection(&self, id: &str) -> Result<types::ConnectionEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_connection".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a connection
    ///
    /// Calls `PUT /nifi-api/connections/{id}`.
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
    /// Requires `Write Source - /{component-type}/{uuid}`.
    /// Requires `Write Destination - /{component-type}/{uuid}`.
    /// Requires `Write New Destination - /{component-type}/{uuid} - if updating Destination`.
    /// Requires `Write Process Group - /process-groups/{uuid} - if updating Destination`.
    async fn update_connection(
        &self,
        id: &str,
        body: types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_connection".to_string(),
            version: "unknown".to_string(),
        })
    }
}
