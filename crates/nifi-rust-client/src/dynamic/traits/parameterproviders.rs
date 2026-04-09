// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for ParameterProvidersApplyParametersRequestsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersApplyParametersRequestsApi {
    /// Deletes the Apply Parameters Request with the given ID
    ///
    /// Deletes the Apply Parameters Request with the given ID. After a request is created via a POST to /nifi-api/parameter-providers/apply-parameters-requests, it is expected that the client will properly clean up the request by DELETE'ing it, once the Apply process has completed. If the request is deleted before the request completes, then the Apply Parameters Request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/parameter-providers/{providerId}/apply-parameters-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Apply Parameters Request
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
    async fn delete_apply_parameters_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_apply_parameters_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Apply Parameters Request with the given ID
    ///
    /// Returns the Apply Parameters Request with the given ID. Once an Apply Parameters Request has been created by performing a POST to /nifi-api/parameter-providers, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/parameter-providers/{providerId}/apply-parameters-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Apply Parameters Request
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
    async fn get_parameter_provider_apply_parameters_request(
        &self,
        request_id: &str,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_provider_apply_parameters_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Initiate a request to apply the fetched parameters of a Parameter Provider
    ///
    /// This will initiate the process of applying fetched parameters to all referencing Parameter Contexts. Changing the value of a Parameter may require that one or more components be stopped and restarted, so this action may take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterProviderApplyParametersRequestEntity, and the process of updating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-providers/apply-parameters-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-providers/apply-parameters-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-providers/{providerId}/apply-parameters-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-providers/{parameterProviderId}`.
    /// Requires `Write - /parameter-providers/{parameterProviderId}`.
    /// Requires `Read - for every component that is affected by the update`.
    /// Requires `Write - for every component that is affected by the update`.
    async fn submit_apply_parameters(
        &self,
        body: &types::ParameterProviderParameterApplicationEntity,
    ) -> Result<types::ParameterProviderApplyParametersRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_apply_parameters".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterProvidersBulletinsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersBulletinsApi {
    /// Clears bulletins for a parameter provider
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/bulletins/clear-requests`.
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
    async fn clear_bulletins_4(
        &self,
        body: &types::ClearBulletinsRequestEntity,
    ) -> Result<types::ClearBulletinsResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins_4".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterProvidersConfigApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersConfigApi {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/config/analysis`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-providers/{uuid}`.
    async fn analyze_configuration_1(
        &self,
        body: &types::ConfigurationAnalysisEntity,
    ) -> Result<types::ConfigurationAnalysisDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "analyze_configuration_1".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/parameter-providers/{id}/config/verification-requests/{requestId}`.
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
    async fn delete_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_verification_request_1".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/parameter-providers/{id}/config/verification-requests/{requestId}`.
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
    async fn get_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_verification_request_1".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs verification of the Parameter Provider's configuration
    ///
    /// This will initiate the process of verifying a given Parameter Provider configuration. This may be a long-running task. As a result, this endpoint will immediately return a ParameterProviderConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-providers/{serviceId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-providers/{providerId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/config/verification-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-providers/{uuid}`.
    async fn submit_config_verification_request_1(
        &self,
        body: &types::VerifyConfigRequestEntity,
    ) -> Result<types::VerifyConfigRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_config_verification_request_1".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterProvidersDescriptorsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersDescriptorsApi {
    /// Gets a parameter provider property descriptor
    ///
    /// Calls `GET /nifi-api/parameter-providers/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `property_name`: The property name.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-providers/{uuid}`.
    async fn get_property_descriptor_2(
        &self,
        property_name: &str,
    ) -> Result<types::PropertyDescriptorDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_property_descriptor_2".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterProvidersParametersApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersParametersApi {
    /// Fetches and temporarily caches the parameters for a provider
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/parameters/fetch-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /parameter-providers/{uuid} or  or /operation/parameter-providers/{uuid}`.
    async fn fetch_parameters(
        &self,
        body: &types::ParameterProviderParameterFetchEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "fetch_parameters".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterProvidersReferencesApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersReferencesApi {
    /// Gets all references to a parameter provider
    ///
    /// Calls `GET /nifi-api/parameter-providers/{id}/references`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-providers/{uuid}`.
    async fn get_parameter_provider_references(
        &self,
    ) -> Result<types::ParameterProviderReferencingComponentsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_provider_references".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterProvidersStateApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersStateApi {
    /// Clears the state for a parameter provider
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/state/clear-requests`.
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
    async fn clear_state_2(
        &self,
        body: &types::ComponentStateEntity,
    ) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_state_2".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the state for a parameter provider
    ///
    /// Calls `GET /nifi-api/parameter-providers/{id}/state`.
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
    async fn get_state_1(&self) -> Result<types::ComponentStateDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_state_1".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The ParameterProviders API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterProvidersApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `provider_id`: The ID of the Parameter Provider
    type ParameterProvidersApplyParametersRequestsApi<
        'b,
    >: ParameterProvidersApplyParametersRequestsApi
    where
        Self: 'b;
    fn apply_parameters_requests<'b>(
        &'b self,
        provider_id: &'b str,
    ) -> Self::ParameterProvidersApplyParametersRequestsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    type ParameterProvidersBulletinsApi<'b>: ParameterProvidersBulletinsApi
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersBulletinsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    type ParameterProvidersConfigApi<'b>: ParameterProvidersConfigApi
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersConfigApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    type ParameterProvidersDescriptorsApi<'b>: ParameterProvidersDescriptorsApi
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersDescriptorsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    type ParameterProvidersParametersApi<'b>: ParameterProvidersParametersApi
    where
        Self: 'b;
    fn parameters<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersParametersApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    type ParameterProvidersReferencesApi<'b>: ParameterProvidersReferencesApi
    where
        Self: 'b;
    fn references<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersReferencesApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    type ParameterProvidersStateApi<'b>: ParameterProvidersStateApi
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ParameterProvidersStateApi<'b>;
    /// Gets a parameter provider
    ///
    /// Calls `GET /nifi-api/parameter-providers/{id}`.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-providers/{uuid}`.
    async fn get_parameter_provider(
        &self,
        id: &str,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_provider".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes a parameter provider
    ///
    /// Calls `DELETE /nifi-api/parameter-providers/{id}`.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
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
    /// Requires `Write - /parameter-providers/{uuid}`.
    /// Requires `Write - /controller`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    async fn remove_parameter_provider(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_parameter_provider".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a parameter provider
    ///
    /// Calls `PUT /nifi-api/parameter-providers/{id}`.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
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
    /// Requires `Read - any referenced Controller Services if this request changes the reference - /controller-services/{uuid}`.
    async fn update_parameter_provider(
        &self,
        id: &str,
        body: &types::ParameterProviderEntity,
    ) -> Result<types::ParameterProviderEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_parameter_provider".to_string(),
            version: "unknown".to_string(),
        })
    }
}
