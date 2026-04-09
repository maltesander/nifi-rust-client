// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for OutputPortsBulletinsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait OutputPortsBulletinsApi {
    /// Clears bulletins for an output port
    ///
    /// Calls `POST /nifi-api/output-ports/{id}/bulletins/clear-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /output-ports/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_bulletins_3(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins_3".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for OutputPortsRunStatusApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait OutputPortsRunStatusApi {
    /// Updates run status of an output-port
    ///
    /// Calls `PUT /nifi-api/output-ports/{id}/run-status`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /output-ports/{uuid} or /operation/output-ports/{uuid}`.
    async fn update_run_status_3(
        &self,
        body: &types::PortRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_run_status_3".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The OutputPorts API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait OutputPortsApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The output port id.
    type OutputPortsBulletinsApi<'b>: OutputPortsBulletinsApi
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::OutputPortsBulletinsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The port id.
    type OutputPortsRunStatusApi<'b>: OutputPortsRunStatusApi
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::OutputPortsRunStatusApi<'b>;
    /// Gets an output port
    ///
    /// Calls `GET /nifi-api/output-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The output port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /output-ports/{uuid}`.
    async fn get_output_port(&self, id: &str) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_output_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes an output port
    ///
    /// Calls `DELETE /nifi-api/output-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The output port id.
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
    /// Requires `Write - /output-ports/{uuid}`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    async fn remove_output_port(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_output_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates an output port
    ///
    /// Calls `PUT /nifi-api/output-ports/{id}`.
    ///
    /// # Parameters
    /// - `id`: The output port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /output-ports/{uuid}`.
    async fn update_output_port(
        &self,
        id: &str,
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_output_port".to_string(),
            version: "unknown".to_string(),
        })
    }
}
