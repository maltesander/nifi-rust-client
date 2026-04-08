// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Processors API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessorsApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/processors/{id}/config/analysis`.
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
    /// Requires `Read - /processors/{uuid}`.
    async fn analyze_configuration_2(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "analyze_configuration_2".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears bulletins for a processor
    ///
    /// Calls `POST /nifi-api/processors/{id}/bulletins/clear-requests`.
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
    /// Requires `Write - /processors/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_bulletins_5(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins_5".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears the state for a processor
    ///
    /// Calls `POST /nifi-api/processors/{id}/state/clear-requests`.
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
    /// Requires `Write - /processors/{uuid}`.
    async fn clear_state_3(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_state_3".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a processor
    ///
    /// Calls `DELETE /nifi-api/processors/{id}`.
    ///
    /// # Parameters
    /// - `id`: The processor id.
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
    /// Requires `Write - /processors/{uuid}`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_processor".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/processors/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Processor
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
    async fn delete_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_verification_request_2".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a processor
    ///
    /// Calls `GET /nifi-api/processors/{id}`.
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
    /// Requires `Read - /processors/{uuid}`.
    async fn get_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processor".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets diagnostics information about a processor
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/processors/{id}/diagnostics`.
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
    /// Requires `Read - /processors/{uuid}`.
    async fn get_processor_diagnostics(
        &self,
        id: &str,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processor_diagnostics".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Submits a query to retrieve the run status details of all processors that are in the given list of Processor IDs
    ///
    /// Calls `POST /nifi-api/processors/run-status-details/queries`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /processors/{uuid} for each processor whose run status information is requested`.
    async fn get_processor_run_status_details(
        &self,
        body: types::RunStatusDetailsRequestEntity,
    ) -> Result<types::ProcessorsRunStatusDetailsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processor_run_status_details".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the descriptor for a processor property
    ///
    /// Calls `GET /nifi-api/processors/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `id`: The processor id.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
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
    /// Requires `Read - /processors/{uuid}`.
    async fn get_property_descriptor_3(
        &self,
        id: &str,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_property_descriptor_3".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the state for a processor
    ///
    /// Calls `GET /nifi-api/processors/{id}/state`.
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
    /// Requires `Write - /processors/{uuid}`.
    async fn get_state_2(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_state_2".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/processors/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Processor
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
    async fn get_verification_request_2(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_verification_request_2".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs verification of the Processor's configuration
    ///
    /// This will initiate the process of verifying a given Processor configuration. This may be a long-running task. As a result, this endpoint will immediately return a ProcessorConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /processors/{processorId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /processors/{processorId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/processors/{id}/config/verification-requests`.
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
    /// Requires `Read - /processors/{uuid}`.
    async fn submit_processor_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_processor_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Terminates a processor, essentially "deleting" its threads and any active tasks
    ///
    /// Calls `DELETE /nifi-api/processors/{id}/threads`.
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
    /// Requires `Write - /processors/{uuid} or /operation/processors/{uuid}`.
    async fn terminate_processor(&self, id: &str) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "terminate_processor".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a processor
    ///
    /// Calls `PUT /nifi-api/processors/{id}`.
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
    /// Requires `Write - /processors/{uuid}`.
    /// Requires `Read - any referenced Controller Services if this request changes the reference - /controller-services/{uuid}`.
    async fn update_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_processor".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates run status of a processor
    ///
    /// Calls `PUT /nifi-api/processors/{id}/run-status`.
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
    /// Requires `Write - /processors/{uuid} or /operation/processors/{uuid}`.
    async fn update_run_status_4(
        &self,
        id: &str,
        body: types::ProcessorRunStatusEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_run_status_4".to_string(),
            version: "unknown".to_string(),
        })
    }
}
