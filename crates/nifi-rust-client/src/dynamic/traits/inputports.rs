// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for InputPortsBulletinsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait InputPortsBulletinsApi {
    /// Clears bulletins for an input port
    ///
    /// Calls `POST /nifi-api/input-ports/{id}/bulletins/clear-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /input-ports/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_bulletins_2(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins_2".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for InputPortsRunStatusApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait InputPortsRunStatusApi {
    /// Updates run status of an input-port
    ///
    /// Calls `PUT /nifi-api/input-ports/{id}/run-status`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /input-ports/{uuid} or /operation/input-ports/{uuid}`.
    async fn update_run_status_2(
        &self,
        body: &types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_run_status_2".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The InputPorts API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait InputPortsApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The input port id.
    fn bulletins<'b>(&'b self, id: &'b str) -> impl InputPortsBulletinsApi + 'b;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The port id.
    fn run_status<'b>(&'b self, id: &'b str) -> impl InputPortsRunStatusApi + 'b;
    /// Gets an input port
    ///
    /// Calls `GET /nifi-api/input-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The input port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /input-ports/{uuid}`.
    async fn get_input_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_input_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes an input port
    ///
    /// Calls `DELETE /nifi-api/input-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The input port id.
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
    /// Requires `Write - /input-ports/{uuid}`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    async fn remove_input_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_input_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates an input port
    ///
    /// Calls `PUT /nifi-api/input-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The input port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /input-ports/{uuid}`.
    async fn update_input_port(
        &self,
        id: &str,
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_input_port".to_string(),
            version: "unknown".to_string(),
        })
    }
}
