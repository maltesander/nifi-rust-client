// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The ReportingTasks API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ReportingTasksApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /reporting-tasks/{uuid}`.
    async fn analyze_configuration_3(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "analyze_configuration_3".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears bulletins for a reporting task
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /reporting-tasks/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_bulletins_7(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins_7".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears the state for a reporting task
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/state/clear-requests`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /reporting-tasks/{uuid}`.
    async fn clear_state_4(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_state_4".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/reporting-tasks/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Reporting Task
    /// - `request_id`: The ID of the Verification Request
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
    async fn delete_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_verification_request_3".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a reporting task property descriptor
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    /// - `property_name`: The property name.
    /// - `sensitive`: Property Descriptor requested sensitive status
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /reporting-tasks/{uuid}`.
    async fn get_property_descriptor_4(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_property_descriptor_4".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a reporting task
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /reporting-tasks/{uuid}`.
    async fn get_reporting_task(&self, id: &str) -> Result<types::ReportingTaskEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_reporting_task".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the state for a reporting task
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}/state`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /reporting-tasks/{uuid}`.
    async fn get_state_4(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_state_4".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Reporting Task
    /// - `request_id`: The ID of the Verification Request
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
    async fn get_verification_request_3(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_verification_request_3".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a reporting task
    ///
    /// Calls `DELETE /nifi-api/reporting-tasks/{id}`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
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
    /// Requires `Write - /reporting-tasks/{uuid}`.
    /// Requires `Write - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_reporting_task".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs verification of the Reporting Task's configuration
    ///
    /// This will initiate the process of verifying a given Reporting Task configuration. This may be a long-running task. As a result, this endpoint will immediately return a ReportingTaskConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /reporting-tasks/{taskId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /reporting-tasks/{serviceId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /reporting-tasks/{uuid}`.
    async fn submit_config_verification_request_2(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_config_verification_request_2".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a reporting task
    ///
    /// Calls `PUT /nifi-api/reporting-tasks/{id}`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /reporting-tasks/{uuid}`.
    /// Requires `Read - any referenced Controller Services if this request changes the reference - /controller-services/{uuid}`.
    async fn update_reporting_task(
        &self,
        id: &str,
        body: types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_reporting_task".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates run status of a reporting task
    ///
    /// Calls `PUT /nifi-api/reporting-tasks/{id}/run-status`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /reporting-tasks/{uuid} or  or /operation/reporting-tasks/{uuid}`.
    async fn update_run_status_5(
        &self,
        id: &str,
        body: types::ReportingTaskRunStatusEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_run_status_5".to_string(),
            version: "unknown".to_string(),
        })
    }
}
