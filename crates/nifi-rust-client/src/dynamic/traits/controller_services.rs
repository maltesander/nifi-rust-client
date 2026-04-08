// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The Controller Services API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerServicesApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller-services/{uuid}`.
    async fn analyze_configuration(
        &self,
        id: &str,
        body: types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "analyze_configuration".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears bulletins for a controller service
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller-services/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_bulletins(
        &self,
        id: &str,
        body: types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears the state for a controller service
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/state/clear-requests`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller-services/{uuid}`.
    async fn clear_state_1(
        &self,
        id: &str,
        body: types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_state_1".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/controller-services/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Controller Service
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
    async fn delete_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a controller service
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/controller-services/{id}`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    /// - `ui_only`
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller-services/{uuid}`.
    async fn get_controller_service(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_service".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a controller service
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/references`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller-services/{uuid}`.
    async fn get_controller_service_references(
        &self,
        id: &str,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_service_references".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a controller service property descriptor
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    /// - `property_name`: The property name to return the descriptor for.
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
    /// Requires `Read - /controller-services/{uuid}`.
    async fn get_property_descriptor_1(
        &self,
        id: &str,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_property_descriptor_1".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the state for a controller service
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/state`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller-services/{uuid}`.
    async fn get_state(&self, id: &str) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_state".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/controller-services/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Controller Service
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
    async fn get_verification_request(
        &self,
        id: &str,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a controller service
    ///
    /// Calls `DELETE /nifi-api/controller-services/{id}`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
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
    /// Requires `Write - /controller-services/{uuid}`.
    /// Requires `Write - Parent Process Group if scoped by Process Group - /process-groups/{uuid}`.
    /// Requires `Write - Controller if scoped by Controller - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    async fn remove_controller_service(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_controller_service".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs verification of the Controller Service's configuration
    ///
    /// This will initiate the process of verifying a given Controller Service configuration. This may be a long-running task. As a result, this endpoint will immediately return a ControllerServiceConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /controller-services/{serviceId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /controller-services/{serviceId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/controller-services/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller-services/{uuid}`.
    async fn submit_config_verification_request(
        &self,
        id: &str,
        body: types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_config_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a controller service
    ///
    /// Calls `PUT /nifi-api/controller-services/{id}`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller-services/{uuid}`.
    /// Requires `Read - any referenced Controller Services if this request changes the reference - /controller-services/{uuid}`.
    async fn update_controller_service(
        &self,
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_controller_service".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a controller services references
    ///
    /// Calls `PUT /nifi-api/controller-services/{id}/references`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /{component-type}/{uuid} or /operate/{component-type}/{uuid} - For each referencing component specified`.
    async fn update_controller_service_references(
        &self,
        id: &str,
        body: types::UpdateControllerServiceReferenceRequestEntity,
    ) -> Result<types::ControllerServiceReferencingComponentsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_controller_service_references".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates run status of a controller service
    ///
    /// Calls `PUT /nifi-api/controller-services/{id}/run-status`.
    ///
    /// # Parameters
    /// - `id`: The controller service id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller-services/{uuid} or /operation/controller-services/{uuid}`.
    async fn update_run_status_1(
        &self,
        id: &str,
        body: types::ControllerServiceRunStatusEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_run_status_1".to_string(),
            version: "unknown".to_string(),
        })
    }
}
