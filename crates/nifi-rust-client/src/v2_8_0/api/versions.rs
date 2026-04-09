// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct VersionsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> VersionsApi<'a> {
    /// Create a version control request
    ///
    /// Creates a request so that a Process Group can be placed under Version Control or have its Version Control configuration changed. Creating this request will prevent any other threads from simultaneously saving local changes to Version Control. It will not, however, actually save the local flow to the Flow Registry. A POST to /versions/process-groups/{id} should be used to initiate saving of the local flow to the Flow Registry. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/active-requests`.
    ///
    /// # Parameters
    /// - `body`: The versioned flow details.
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
    pub async fn create_version_control_request(
        &self,
        body: &crate::v2_8_0::types::CreateActiveRequestEntity,
    ) -> Result<(), NifiError> {
        self.client
            .post_void("/versions/active-requests", body)
            .await
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
    pub async fn delete_version_control_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<(), NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_with_query(&format!("/versions/active-requests/{id}"), &query)
            .await
    }
    /// Updates the request with the given ID
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/versions/active-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The request ID.
    /// - `body`: The version control component mapping.
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
    pub async fn update_version_control_request(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionControlComponentMappingEntity,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.client
            .put(&format!("/versions/active-requests/{id}"), body)
            .await
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
    /// - `Read - /process-groups/{uuid}`
    /// - `Write - /process-groups/{uuid}`
    pub async fn stop_version_control(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/versions/process-groups/{id}"), &query)
            .await
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
    pub async fn get_version_information(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.client
            .get(&format!("/versions/process-groups/{id}"))
            .await
    }
    /// Save the Process Group with the given ID
    ///
    /// Begins version controlling the Process Group with the given ID or commits changes to the Versioned Flow, depending on if the provided VersionControlInformation includes a flowId. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `body`: The versioned flow details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /process-groups/{uuid}`
    /// - `Write - /process-groups/{uuid}`
    /// - `Read - /{component-type}/{uuid} - For all encapsulated components`
    /// - `Read - any referenced Controller Services by any encapsulated components - /controller-services/{uuid}`
    pub async fn save_to_flow_registry(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::StartVersionControlRequestEntity,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.client
            .post(&format!("/versions/process-groups/{id}"), body)
            .await
    }
    /// Update the version of a Process Group with the given ID
    ///
    /// For a Process Group that is already under Version Control, this will update the version of the flow to a different version. This endpoint expects that the given snapshot will not modify any Processor that is currently running or any Controller Service that is enabled. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/versions/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `body`: The controller service configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /process-groups/{uuid}`
    /// - `Write - /process-groups/{uuid}`
    pub async fn update_flow_version(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionedFlowSnapshotEntity,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.client
            .put(&format!("/versions/process-groups/{id}"), body)
            .await
    }
    /// Initiate the Revert Request of a Process Group with the given ID
    ///
    /// For a Process Group that is already under Version Control, this will initiate the action of reverting any local changes that have been made to the Process Group since it was last synchronized with the Flow Registry. This will result in the flow matching the Versioned Flow that exists in the Flow Registry. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a VersionedFlowUpdateRequestEntity, and the process of updating the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /versions/revert-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /versions/revert-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/revert-requests/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `body`: The Version Control Information to revert to.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /process-groups/{uuid}`
    /// - `Write - /process-groups/{uuid}`
    /// - `Read - /{component-type}/{uuid} - For all encapsulated components`
    /// - `Write - /{component-type}/{uuid} - For all encapsulated components`
    /// - `Write - if the template contains any restricted components - /restricted-components`
    /// - `Read - /parameter-contexts/{uuid} - For any Parameter Context that is referenced by a Property that is changed, added, or removed`
    pub async fn initiate_revert_flow_version(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionControlInformationEntity,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.client
            .post(
                &format!("/versions/revert-requests/process-groups/{id}"),
                body,
            )
            .await
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
    pub async fn delete_revert_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/versions/revert-requests/{id}"), &query)
            .await
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
    pub async fn get_revert_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.client
            .get(&format!("/versions/revert-requests/{id}"))
            .await
    }
    /// Initiate the Update Request of a Process Group with the given ID
    ///
    /// For a Process Group that is already under Version Control, this will initiate the action of changing from a specific version of the flow in the Flow Registry to a different version of the flow. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a VersionedFlowUpdateRequestEntity, and the process of updating the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /versions/update-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /versions/update-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/versions/update-requests/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `body`: The controller service configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /process-groups/{uuid}`
    /// - `Write - /process-groups/{uuid}`
    /// - `Read - /{component-type}/{uuid} - For all encapsulated components`
    /// - `Write - /{component-type}/{uuid} - For all encapsulated components`
    /// - `Write - if the template contains any restricted components - /restricted-components`
    /// - `Read - /parameter-contexts/{uuid} - For any Parameter Context that is referenced by a Property that is changed, added, or removed`
    pub async fn initiate_version_control_update(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionControlInformationEntity,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.client
            .post(
                &format!("/versions/update-requests/process-groups/{id}"),
                body,
            )
            .await
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
    pub async fn delete_update_request_1(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/versions/update-requests/{id}"), &query)
            .await
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
    pub async fn get_update_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.client
            .get(&format!("/versions/update-requests/{id}"))
            .await
    }
    /// Scope operations to the `download` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn download<'b>(&'b self, id: &'b str) -> VersionsDownloadApi<'b> {
        VersionsDownloadApi {
            client: self.client,
            id,
        }
    }
}
pub struct VersionsDownloadApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> VersionsDownloadApi<'a> {
    /// Gets the latest version of a Process Group for download
    ///
    /// Calls `GET /nifi-api/versions/process-groups/{id}/download`.
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
    pub async fn export_flow_version(&self) -> Result<(), NifiError> {
        let id = self.id;
        self.client
            .get_void(&format!("/versions/process-groups/{id}/download"))
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::VersionsApi for VersionsApi<'_> {
    type VersionsDownloadApi<'b>
        = VersionsDownloadApi<'b>
    where
        Self: 'b;
    fn download<'b>(&'b self, id: &'b str) -> Self::VersionsDownloadApi<'b> {
        VersionsDownloadApi {
            client: self.client,
            id,
        }
    }
    async fn create_version_control_request(
        &self,
        body: &crate::v2_8_0::types::CreateActiveRequestEntity,
    ) -> Result<(), NifiError> {
        self.create_version_control_request(body).await
    }
    async fn delete_version_control_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<(), NifiError> {
        self.delete_version_control_request(id, disconnected_node_acknowledged)
            .await
    }
    async fn update_version_control_request(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionControlComponentMappingEntity,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.update_version_control_request(id, body).await
    }
    async fn stop_version_control(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.stop_version_control(id, version, client_id, disconnected_node_acknowledged)
            .await
    }
    async fn get_version_information(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.get_version_information(id).await
    }
    async fn save_to_flow_registry(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::StartVersionControlRequestEntity,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.save_to_flow_registry(id, body).await
    }
    async fn update_flow_version(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionedFlowSnapshotEntity,
    ) -> Result<crate::v2_8_0::types::VersionControlInformationEntity, NifiError> {
        self.update_flow_version(id, body).await
    }
    async fn initiate_revert_flow_version(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionControlInformationEntity,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.initiate_revert_flow_version(id, body).await
    }
    async fn delete_revert_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.delete_revert_request(id, disconnected_node_acknowledged)
            .await
    }
    async fn get_revert_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.get_revert_request(id).await
    }
    async fn initiate_version_control_update(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::VersionControlInformationEntity,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.initiate_version_control_update(id, body).await
    }
    async fn delete_update_request_1(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.delete_update_request_1(id, disconnected_node_acknowledged)
            .await
    }
    async fn get_update_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::VersionedFlowUpdateRequestEntity, NifiError> {
        self.get_update_request(id).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_8_0::traits::VersionsDownloadApi for VersionsDownloadApi<'_> {
    async fn export_flow_version(&self) -> Result<(), NifiError> {
        self.export_flow_version().await
    }
}
