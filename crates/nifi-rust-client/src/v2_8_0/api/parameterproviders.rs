// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ParameterProvidersApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersApi<'a> {
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
    /// - `Write - /parameter-providers/{uuid}`
    /// - `Write - /controller`
    /// - `Read - any referenced Controller Services - /controller-services/{uuid}`
    pub async fn remove_parameter_provider(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ParameterProviderEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/parameter-providers/{id}"), &query)
            .await
    }
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
    pub async fn get_parameter_provider(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::ParameterProviderEntity, NifiError> {
        self.client.get(&format!("/parameter-providers/{id}")).await
    }
    /// Updates a parameter provider
    ///
    /// Calls `PUT /nifi-api/parameter-providers/{id}`.
    ///
    /// # Parameters
    /// - `id`: The parameter provider id.
    /// - `body`: The parameter provider configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Write - /parameter-providers/{uuid}`
    /// - `Read - any referenced Controller Services if this request changes the reference - /controller-services/{uuid}`
    pub async fn update_parameter_provider(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::ParameterProviderEntity,
    ) -> Result<crate::v2_8_0::types::ParameterProviderEntity, NifiError> {
        self.client
            .put(&format!("/parameter-providers/{id}"), body)
            .await
    }
    /// Scope operations to the `apply_parameters_requests` sub-resource of a specific process group.
    ///
    /// - `provider_id`: The ID of the Parameter Provider
    pub fn apply_parameters_requests<'b>(
        &'b self,
        provider_id: &'b str,
    ) -> ParameterProvidersApplyParametersRequestsApi<'b> {
        ParameterProvidersApplyParametersRequestsApi {
            client: self.client,
            provider_id,
        }
    }
    /// Scope operations to the `bulletins` sub-resource of a specific process group.
    ///
    /// - `id`: The parameter provider id.
    pub fn bulletins<'b>(&'b self, id: &'b str) -> ParameterProvidersBulletinsApi<'b> {
        ParameterProvidersBulletinsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `config` sub-resource of a specific process group.
    ///
    /// - `id`: The parameter provider id.
    pub fn config<'b>(&'b self, id: &'b str) -> ParameterProvidersConfigApi<'b> {
        ParameterProvidersConfigApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `descriptors` sub-resource of a specific process group.
    ///
    /// - `id`: The parameter provider id.
    pub fn descriptors<'b>(&'b self, id: &'b str) -> ParameterProvidersDescriptorsApi<'b> {
        ParameterProvidersDescriptorsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `parameters` sub-resource of a specific process group.
    ///
    /// - `id`: The parameter provider id.
    pub fn parameters<'b>(&'b self, id: &'b str) -> ParameterProvidersParametersApi<'b> {
        ParameterProvidersParametersApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `references` sub-resource of a specific process group.
    ///
    /// - `id`: The parameter provider id.
    pub fn references<'b>(&'b self, id: &'b str) -> ParameterProvidersReferencesApi<'b> {
        ParameterProvidersReferencesApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `state` sub-resource of a specific process group.
    ///
    /// - `id`: The parameter provider id.
    pub fn state<'b>(&'b self, id: &'b str) -> ParameterProvidersStateApi<'b> {
        ParameterProvidersStateApi {
            client: self.client,
            id,
        }
    }
}
pub struct ParameterProvidersApplyParametersRequestsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) provider_id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersApplyParametersRequestsApi<'a> {
    /// Initiate a request to apply the fetched parameters of a Parameter Provider
    ///
    /// This will initiate the process of applying fetched parameters to all referencing Parameter Contexts. Changing the value of a Parameter may require that one or more components be stopped and restarted, so this action may take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterProviderApplyParametersRequestEntity, and the process of updating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-providers/apply-parameters-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-providers/apply-parameters-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-providers/{providerId}/apply-parameters-requests`.
    ///
    /// # Parameters
    /// - `body`: The apply parameters request.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /parameter-providers/{parameterProviderId}`
    /// - `Write - /parameter-providers/{parameterProviderId}`
    /// - `Read - for every component that is affected by the update`
    /// - `Write - for every component that is affected by the update`
    pub async fn submit_apply_parameters(
        &self,
        body: &crate::v2_8_0::types::ParameterProviderParameterApplicationEntity,
    ) -> Result<crate::v2_8_0::types::ParameterProviderApplyParametersRequestDto, NifiError> {
        let provider_id = self.provider_id;
        let e: crate::v2_8_0::types::ParameterProviderApplyParametersRequestEntity = self
            .client
            .post(
                &format!("/parameter-providers/{provider_id}/apply-parameters-requests"),
                body,
            )
            .await?;
        Ok(e.request)
    }
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
    pub async fn delete_apply_parameters_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ParameterProviderApplyParametersRequestDto, NifiError> {
        let provider_id = self.provider_id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        let e: crate::v2_8_0::types::ParameterProviderApplyParametersRequestEntity = self
            .client
            .delete_returning_with_query(
                &format!(
                    "/parameter-providers/{provider_id}/apply-parameters-requests/{request_id}"
                ),
                &query,
            )
            .await?;
        Ok(e.request)
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
    pub async fn get_parameter_provider_apply_parameters_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::ParameterProviderApplyParametersRequestDto, NifiError> {
        let provider_id = self.provider_id;
        let e: crate::v2_8_0::types::ParameterProviderApplyParametersRequestEntity = self
            .client
            .get(&format!(
                "/parameter-providers/{provider_id}/apply-parameters-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
}
pub struct ParameterProvidersBulletinsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersBulletinsApi<'a> {
    /// Clears bulletins for a parameter provider
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The clear bulletin request specifying the timestamp from which to clear bulletins.
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
    pub async fn clear_bulletins_4(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/parameter-providers/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
}
pub struct ParameterProvidersConfigApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersConfigApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `body`: The configuration analysis request.
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
    pub async fn analyze_configuration_1(
        &self,
        body: &crate::v2_8_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_8_0::types::ConfigurationAnalysisDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ConfigurationAnalysisEntity = self
            .client
            .post(&format!("/parameter-providers/{id}/config/analysis"), body)
            .await?;
        Ok(e.configuration_analysis)
    }
    /// Performs verification of the Parameter Provider's configuration
    ///
    /// This will initiate the process of verifying a given Parameter Provider configuration. This may be a long-running task. As a result, this endpoint will immediately return a ParameterProviderConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-providers/{serviceId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-providers/{providerId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `body`: The parameter provider configuration verification request.
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
    pub async fn submit_config_verification_request_1(
        &self,
        body: &crate::v2_8_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .post(
                &format!("/parameter-providers/{id}/config/verification-requests"),
                body,
            )
            .await?;
        Ok(e.request)
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
    pub async fn delete_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .delete_returning(&format!(
                "/parameter-providers/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
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
    pub async fn get_verification_request_1(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .get(&format!(
                "/parameter-providers/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
}
pub struct ParameterProvidersDescriptorsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersDescriptorsApi<'a> {
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
    pub async fn get_property_descriptor_2(
        &self,
        property_name: &str,
    ) -> Result<crate::v2_8_0::types::PropertyDescriptorDto, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("propertyName", property_name.to_string()));
        let e: crate::v2_8_0::types::PropertyDescriptorEntity = self
            .client
            .get_with_query(&format!("/parameter-providers/{id}/descriptors"), &query)
            .await?;
        Ok(e.property_descriptor)
    }
}
pub struct ParameterProvidersParametersApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersParametersApi<'a> {
    /// Fetches and temporarily caches the parameters for a provider
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/parameters/fetch-requests`.
    ///
    /// # Parameters
    /// - `body`: The parameter fetch request.
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
    pub async fn fetch_parameters(
        &self,
        body: &crate::v2_8_0::types::ParameterProviderParameterFetchEntity,
    ) -> Result<crate::v2_8_0::types::ParameterProviderEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/parameter-providers/{id}/parameters/fetch-requests"),
                body,
            )
            .await
    }
}
pub struct ParameterProvidersReferencesApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersReferencesApi<'a> {
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
    pub async fn get_parameter_provider_references(
        &self,
    ) -> Result<crate::v2_8_0::types::ParameterProviderReferencingComponentsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/parameter-providers/{id}/references"))
            .await
    }
}
pub struct ParameterProvidersStateApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterProvidersStateApi<'a> {
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
    pub async fn get_state_1(&self) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ComponentStateEntity = self
            .client
            .get(&format!("/parameter-providers/{id}/state"))
            .await?;
        Ok(e.component_state)
    }
    /// Clears the state for a parameter provider
    ///
    /// Calls `POST /nifi-api/parameter-providers/{id}/state/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: Optional component state to perform a selective key removal. If omitted, clears all state.
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
    pub async fn clear_state_2(
        &self,
        body: &crate::v2_8_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ComponentStateEntity = self
            .client
            .post(
                &format!("/parameter-providers/{id}/state/clear-requests"),
                body,
            )
            .await?;
        Ok(e.component_state)
    }
}
