// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for ControllerBulletinsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerBulletinsApi {
    /// Clears bulletins for a flow analysis rule
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/bulletins/clear-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /flow-analysis-rules/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_flow_analysis_rule_bulletins(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_flow_analysis_rule_bulletins".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears bulletins for a parameter provider
    ///
    /// Calls `POST /nifi-api/controller/parameter-providers/{id}/bulletins/clear-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /parameter-providers/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_parameter_provider_bulletins(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_parameter_provider_bulletins".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Clears bulletins for a registry client
    ///
    /// Calls `POST /nifi-api/controller/registry-clients/{id}/bulletins/clear-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller/registry-clients/{uuid}`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_registry_client_bulletins(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_registry_client_bulletins".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ControllerConfigApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerConfigApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/config/analysis`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow-analysis-rules/{uuid}`.
    async fn analyze_flow_analysis_rule_configuration(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "analyze_flow_analysis_rule_configuration".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/controller/registry-clients/{id}/config/analysis`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn analyze_flow_registry_client_configuration(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "analyze_flow_registry_client_configuration".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/controller/flow-analysis-rules/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
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
    async fn delete_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_flow_analysis_rule_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/controller/registry-clients/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
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
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn delete_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_registry_client_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
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
    async fn get_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_rule_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once a Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/controller/registry-clients/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
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
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn get_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_registry_client_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs verification of the Flow Analysis Rule's configuration
    ///
    /// This will initiate the process of verifying a given Flow Analysis Rule configuration. This may be a long-running task. As a result, this endpoint will immediately return a FlowAnalysisRuleConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /flow-analysis-rules/{taskId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /flow-analysis-rules/{serviceId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/config/verification-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow-analysis-rules/{uuid}`.
    async fn submit_flow_analysis_rule_config_verification_request(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_flow_analysis_rule_config_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs verification of the Registry Client's configuration
    ///
    /// Initiates verification of a Registry Client configuration. The request returns immediately with a request entity while verification runs asynchronously. The client should poll /controller/registry-clients/{clientId}/config/verification-requests/{requestId} for status and DELETE the request once verification completes.
    ///
    /// Calls `POST /nifi-api/controller/registry-clients/{id}/config/verification-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn submit_registry_client_config_verification_request(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_registry_client_config_verification_request".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ControllerContentApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerContentApi {
    /// Retrieves the content of the NAR with the given id
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars/{id}/content`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn download_nar(&self) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "download_nar".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ControllerDescriptorsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerDescriptorsApi {
    /// Gets a flow analysis rule property descriptor
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}/descriptors`.
    ///
    /// # Parameters
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
    /// Requires `Read - /flow-analysis-rules/{uuid}`.
    async fn get_flow_analysis_rule_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_rule_property_descriptor".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a flow registry client property descriptor
    ///
    /// Calls `GET /nifi-api/controller/registry-clients/{id}/descriptors`.
    ///
    /// # Parameters
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
    /// Requires `Read - /controller/registry-clients/{id}`.
    async fn get_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_property_descriptor".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ControllerDetailsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerDetailsApi {
    /// Retrieves the component types available from the installed NARs
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars/{id}/details`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_nar_details(&self) -> Result<types::NarDetailsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_nar_details".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ControllerRunStatusApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerRunStatusApi {
    /// Updates run status of a flow analysis rule
    ///
    /// Calls `PUT /nifi-api/controller/flow-analysis-rules/{id}/run-status`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /flow-analysis-rules/{uuid} or  or /operation/flow-analysis-rules/{uuid}`.
    async fn update_run_status(
        &self,
        body: &types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_run_status".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ControllerStateApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerStateApi {
    /// Clears the state for a flow analysis rule
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/state/clear-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /flow-analysis-rules/{uuid}`.
    async fn clear_state(
        &self,
        body: &types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_state".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the state for a flow analysis rule
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}/state`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /flow-analysis-rules/{uuid}`.
    async fn get_flow_analysis_rule_state(&self) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_rule_state".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The Controller API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ControllerApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    type ControllerBulletinsApi<'b>: ControllerBulletinsApi
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ControllerBulletinsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rules id.
    type ControllerConfigApi<'b>: ControllerConfigApi
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ControllerConfigApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The id of the NAR.
    type ControllerContentApi<'b>: ControllerContentApi
    where
        Self: 'b;
    fn content<'b>(&'b self, id: &'b str) -> Self::ControllerContentApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    type ControllerDescriptorsApi<'b>: ControllerDescriptorsApi
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ControllerDescriptorsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The id of the NAR.
    type ControllerDetailsApi<'b>: ControllerDetailsApi
    where
        Self: 'b;
    fn details<'b>(&'b self, id: &'b str) -> Self::ControllerDetailsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    type ControllerRunStatusApi<'b>: ControllerRunStatusApi
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ControllerRunStatusApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    type ControllerStateApi<'b>: ControllerStateApi
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ControllerStateApi<'b>;
    /// Creates a new bulletin
    ///
    /// Calls `POST /nifi-api/controller/bulletin`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn create_bulletin(
        &self,
        body: &types::BulletinEntity,
    ) -> Result<types::BulletinEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_bulletin".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new controller service
    ///
    /// Calls `POST /nifi-api/controller/controller-services`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    /// Requires `Write - if the Controller Service is restricted - /restricted-components`.
    async fn create_controller_service(
        &self,
        body: &types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_controller_service".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new flow analysis rule
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    /// Requires `Write - if the Flow Analysis Rule is restricted - /restricted-components`.
    async fn create_flow_analysis_rule(
        &self,
        body: &types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_flow_analysis_rule".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new flow registry client
    ///
    /// Calls `POST /nifi-api/controller/registry-clients`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    /// Requires `Write - /controller`.
    async fn create_flow_registry_client(
        &self,
        body: &types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_flow_registry_client".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new parameter provider
    ///
    /// Calls `POST /nifi-api/controller/parameter-providers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    /// Requires `Write - if the Parameter Provider is restricted - /restricted-components`.
    async fn create_parameter_provider(
        &self,
        body: &types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_parameter_provider".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new reporting task
    ///
    /// Calls `POST /nifi-api/controller/reporting-tasks`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    /// Requires `Write - if the Reporting Task is restricted - /restricted-components`.
    async fn create_reporting_task(
        &self,
        body: &types::ReportingTaskEntity,
    ) -> Result<types::ReportingTaskEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_reporting_task".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a flow registry client
    ///
    /// Calls `DELETE /nifi-api/controller/registry-clients/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow registry client id.
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
    /// Requires `Write - /controller/registry-clients/{id}`.
    async fn delete_flow_registry_client(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_flow_registry_client".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Purges history
    ///
    /// Calls `DELETE /nifi-api/controller/history`.
    ///
    /// # Parameters
    /// - `end_date`: Purge actions before this date/time.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn delete_history(&self, end_date: &str) -> Result<types::HistoryDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_history".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes an installed NAR
    ///
    /// Calls `DELETE /nifi-api/controller/nar-manager/nars/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the NAR.
    /// - `disconnected_node_acknowledged`
    /// - `force`
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn delete_nar(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
        force: Option<bool>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_nar".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Removes a node from the cluster
    ///
    /// Calls `DELETE /nifi-api/controller/cluster/nodes/{id}`.
    ///
    /// # Parameters
    /// - `id`: The node id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn delete_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_node".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the contents of the cluster
    ///
    /// Returns the contents of the cluster including all nodes and their status.
    ///
    /// Calls `GET /nifi-api/controller/cluster`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_cluster(&self) -> Result<types::ClusterDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_cluster".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the configuration for this NiFi Controller
    ///
    /// Calls `GET /nifi-api/controller/config`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_controller_config(
        &self,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_config".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a flow analysis rule
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow-analysis-rules/{uuid}`.
    async fn get_flow_analysis_rule(
        &self,
        id: &str,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_rule".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all flow analysis rules
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flow_analysis_rules(&self) -> Result<types::FlowAnalysisRulesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_rules".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a flow registry client
    ///
    /// Calls `GET /nifi-api/controller/registry-clients/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow registry client id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller/registry-clients/{id}`.
    async fn get_flow_registry_client(
        &self,
        id: &str,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_registry_client".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the listing of available flow registry clients
    ///
    /// Calls `GET /nifi-api/controller/registry-clients`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_flow_registry_clients(
        &self,
    ) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_registry_clients".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves summary information for installed NARs
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_nar_summaries(&self) -> Result<types::NarSummariesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_nar_summaries".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the summary information for the NAR with the given identifier
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the NAR.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_nar_summary(&self, id: &str) -> Result<types::NarDetailsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_nar_summary".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a node in the cluster
    ///
    /// Calls `GET /nifi-api/controller/cluster/nodes/{id}`.
    ///
    /// # Parameters
    /// - `id`: The node id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_node(&self, id: &str) -> Result<types::NodeDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_node".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets status history for the node
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/controller/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_node_status_history(&self) -> Result<types::ComponentHistoryDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_node_status_history".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the types of flow  that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/controller/registry-types`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /controller`.
    async fn get_registry_client_types(
        &self,
    ) -> Result<types::FlowRegistryClientTypesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_registry_client_types".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Imports a reporting task snapshot
    ///
    /// Calls `POST /nifi-api/controller/reporting-tasks/import`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn import_reporting_task_snapshot(
        &self,
        body: &types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<types::VersionedReportingTaskImportResponseEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "import_reporting_task_snapshot".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a flow analysis rule
    ///
    /// Calls `DELETE /nifi-api/controller/flow-analysis-rules/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
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
    /// Requires `Write - /flow-analysis-rules/{uuid}`.
    /// Requires `Write - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    async fn remove_flow_analysis_rule(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_flow_analysis_rule".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the configuration for this NiFi
    ///
    /// Calls `PUT /nifi-api/controller/config`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn update_controller_config(
        &self,
        body: &types::ControllerConfigurationEntity,
    ) -> Result<types::ControllerConfigurationEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_controller_config".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a flow analysis rule
    ///
    /// Calls `PUT /nifi-api/controller/flow-analysis-rules/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /flow-analysis-rules/{uuid}`.
    /// Requires `Read - any referenced Controller Services if this request changes the reference - /controller-services/{uuid}`.
    async fn update_flow_analysis_rule(
        &self,
        id: &str,
        body: &types::FlowAnalysisRuleEntity,
    ) -> Result<types::FlowAnalysisRuleEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_flow_analysis_rule".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a flow registry client
    ///
    /// Calls `PUT /nifi-api/controller/registry-clients/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow registry client id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller/registry-clients/{id}`.
    async fn update_flow_registry_client(
        &self,
        id: &str,
        body: &types::FlowRegistryClientEntity,
    ) -> Result<types::FlowRegistryClientEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_flow_registry_client".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a node in the cluster
    ///
    /// Calls `PUT /nifi-api/controller/cluster/nodes/{id}`.
    ///
    /// # Parameters
    /// - `id`: The node id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn update_node(
        &self,
        id: &str,
        body: &types::NodeEntity,
    ) -> Result<types::NodeDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_node".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Uploads a NAR and requests for it to be installed
    ///
    /// Calls `POST /nifi-api/controller/nar-manager/nars/content`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /controller`.
    async fn upload_nar(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::NarSummaryDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "upload_nar".to_string(),
            version: "unknown".to_string(),
        })
    }
}
