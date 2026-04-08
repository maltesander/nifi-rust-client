// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The RemoteProcessGroups API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait RemoteProcessGroupsApi {
    /// Clears bulletins for a remote process group
    ///
    /// Calls `POST /nifi-api/remote-process-groups/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_bulletins_6(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins_6".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a remote process group
    ///
    /// Calls `GET /nifi-api/remote-process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /remote-process-groups/{uuid}`.
    async fn get_remote_process_group(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_remote_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the state for a RemoteProcessGroup
    ///
    /// Calls `GET /nifi-api/remote-process-groups/{id}/state`.
    ///
    /// # Parameters
    /// - `id`: The processor id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid}`.
    async fn get_state_3(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_state_3".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a remote process group
    ///
    /// Calls `DELETE /nifi-api/remote-process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
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
    /// Requires `Write - /remote-process-groups/{uuid}`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    async fn remove_remote_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_remote_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a remote process group
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid}`.
    async fn update_remote_process_group(
        &self,
        id: &str,
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_remote_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/input-ports/{port-id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    /// - `port_id`: The remote process group port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid}`.
    async fn update_remote_process_group_input_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_remote_process_group_input_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates run status of a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/input-ports/{port-id}/run-status`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    /// - `port_id`: The remote process group port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid} or /operation/remote-process-groups/{uuid}`.
    async fn update_remote_process_group_input_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_remote_process_group_input_port_run_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/output-ports/{port-id}`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    /// - `port_id`: The remote process group port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid}`.
    async fn update_remote_process_group_output_port(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemoteProcessGroupPortEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_remote_process_group_output_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates run status of a remote port
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/output-ports/{port-id}/run-status`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    /// - `port_id`: The remote process group port id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid} or /operation/remote-process-groups/{uuid}`.
    async fn update_remote_process_group_output_port_run_status(
        &self,
        id: &str,
        port_id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupPortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_remote_process_group_output_port_run_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates run status of a remote process group
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/{id}/run-status`.
    ///
    /// # Parameters
    /// - `id`: The remote process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid} or /operation/remote-process-groups/{uuid}`.
    async fn update_remote_process_group_run_status(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_remote_process_group_run_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates run status of all remote process groups in a process group (recursively)
    ///
    /// Calls `PUT /nifi-api/remote-process-groups/process-group/{id}/run-status`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /remote-process-groups/{uuid} or /operation/remote-process-groups/{uuid}`.
    async fn update_remote_process_group_run_statuses(
        &self,
        id: &str,
        body: types::RemotePortRunStatusEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_remote_process_group_run_statuses".to_string(),
            version: "unknown".to_string(),
        })
    }
}
