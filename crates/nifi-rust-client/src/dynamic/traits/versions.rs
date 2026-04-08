// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Versions API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait VersionsApi {
    /// Create a version control request
    ///
    /// Creates a request so that a Process Group can be placed under Version Control or have its Version Control configuration changed. Creating this request will prevent any other threads from simultaneously saving local changes to Version Control. It will not, however, actually save the local flow to the Flow Registry. A POST to /versions/process-groups/{id} should be used to initiate saving of the local flow to the Flow Registry. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/active-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn create_version_control_request(
        &self,
        body: types::CreateActiveRequestEntity,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_version_control_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Revert Request with the given ID
    ///
    /// Deletes the Revert Request with the given ID. After a request is created via a POST to /versions/revert-requests/process-groups/{id}, it is expected that the client will properly clean up the request by DELETE'ing it, once the Revert process has completed. If the request is deleted before the request completes, then the Revert request will finish the step that it is currently performing and then will cancel any subsequent steps. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/versions/revert-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Revert Request
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
    /// Requires `Only the user that submitted the request can remove it`.
    async fn delete_revert_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_revert_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Update Request with the given ID
    ///
    /// Deletes the Update Request with the given ID. After a request is created via a POST to /versions/update-requests/process-groups/{id}, it is expected that the client will properly clean up the request by DELETE'ing it, once the Update process has completed. If the request is deleted before the request completes, then the Update request will finish the step that it is currently performing and then will cancel any subsequent steps. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/versions/update-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Update Request
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
    /// Requires `Only the user that submitted the request can remove it`.
    async fn delete_update_request_1(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_update_request_1".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the version control request with the given ID
    ///
    /// Deletes the Version Control Request with the given ID. This will allow other threads to save flows to the Flow Registry. See also the documentation for POSTing to /versions/active-requests for information regarding why this is done. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/versions/active-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The request ID.
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
    /// Requires `Only the user that submitted the request can remove it`.
    async fn delete_version_control_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_version_control_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the latest version of a Process Group for download
    ///
    /// Calls `GET /nifi-api/versions/process-groups/{id}/download`.
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
    /// Requires `Read - /process-groups/{uuid}`.
    async fn export_flow_version(&self, id: &str) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "export_flow_version".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Revert Request with the given ID
    ///
    /// Returns the Revert Request with the given ID. Once a Revert Request has been created by performing a POST to /versions/revert-requests/process-groups/{id}, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/versions/revert-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Revert Request
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Only the user that submitted the request can get it`.
    async fn get_revert_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_revert_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Update Request with the given ID
    ///
    /// Returns the Update Request with the given ID. Once an Update Request has been created by performing a POST to /versions/update-requests/process-groups/{id}, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/versions/update-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Update Request
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Only the user that submitted the request can get it`.
    async fn get_update_request(
        &self,
        id: &str,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_update_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the Version Control information for a process group
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/versions/process-groups/{id}`.
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
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_version_information(
        &self,
        id: &str,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_version_information".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Initiate the Revert Request of a Process Group with the given ID
    ///
    /// For a Process Group that is already under Version Control, this will initiate the action of reverting any local changes that have been made to the Process Group since it was last synchronized with the Flow Registry. This will result in the flow matching the Versioned Flow that exists in the Flow Registry. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a VersionedFlowUpdateRequestEntity, and the process of updating the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /versions/revert-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /versions/revert-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/revert-requests/process-groups/{id}`.
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
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - if the template contains any restricted components - /restricted-components`.
    /// Requires `Read - /parameter-contexts/{uuid} - For any Parameter Context that is referenced by a Property that is changed, added, or removed`.
    async fn initiate_revert_flow_version(
        &self,
        id: &str,
        body: types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "initiate_revert_flow_version".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Initiate the Update Request of a Process Group with the given ID
    ///
    /// For a Process Group that is already under Version Control, this will initiate the action of changing from a specific version of the flow in the Flow Registry to a different version of the flow. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a VersionedFlowUpdateRequestEntity, and the process of updating the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /versions/update-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /versions/update-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/update-requests/process-groups/{id}`.
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
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - if the template contains any restricted components - /restricted-components`.
    /// Requires `Read - /parameter-contexts/{uuid} - For any Parameter Context that is referenced by a Property that is changed, added, or removed`.
    async fn initiate_version_control_update(
        &self,
        id: &str,
        body: types::VersionControlInformationEntity,
    ) -> Result<types::VersionedFlowUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "initiate_version_control_update".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Save the Process Group with the given ID
    ///
    /// Begins version controlling the Process Group with the given ID or commits changes to the Versioned Flow, depending on if the provided VersionControlInformation includes a flowId. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/process-groups/{id}`.
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
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Read - any referenced Controller Services by any encapsulated components - /controller-services/{uuid}`.
    async fn save_to_flow_registry(
        &self,
        id: &str,
        body: types::StartVersionControlRequestEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "save_to_flow_registry".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Stops version controlling the Process Group with the given ID
    ///
    /// Stops version controlling the Process Group with the given ID. The Process Group will no longer track to any Versioned Flow. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/versions/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `version`: The version is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, a new one will be generated. This value (whether specified or generated) is included in the response.
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
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Write - /process-groups/{uuid}`.
    async fn stop_version_control(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "stop_version_control".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Update the version of a Process Group with the given ID
    ///
    /// For a Process Group that is already under Version Control, this will update the version of the flow to a different version. This endpoint expects that the given snapshot will not modify any Processor that is currently running or any Controller Service that is enabled. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/versions/process-groups/{id}`.
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
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Write - /process-groups/{uuid}`.
    async fn update_flow_version(
        &self,
        id: &str,
        body: types::VersionedFlowSnapshotEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_flow_version".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates the request with the given ID
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/versions/active-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The request ID.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Only the user that submitted the request can update it`.
    async fn update_version_control_request(
        &self,
        id: &str,
        body: types::VersionControlComponentMappingEntity,
    ) -> Result<types::VersionControlInformationEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_version_control_request".to_string(),
            version: "unknown".to_string(),
        })
    }
}
