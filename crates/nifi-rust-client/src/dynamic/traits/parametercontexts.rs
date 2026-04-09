// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for ParameterContextsAssetsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsAssetsApi {
    /// Creates a new Asset in the given Parameter Context
    ///
    /// This endpoint will create a new Asset in the given Parameter Context. The Asset will be created with the given name and the contents of the file that is uploaded. The Asset will be created in the given Parameter Context, and will be available for use by any component that references the Parameter Context.
    ///
    /// Calls `POST /nifi-api/parameter-contexts/{contextId}/assets`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{parameterContextId}`.
    /// Requires `Write - /parameter-contexts/{parameterContextId}`.
    /// Requires `Read - for every component that is affected by the update`.
    /// Requires `Write - for every component that is affected by the update`.
    /// Requires `Read - for every currently inherited parameter context`.
    async fn create_asset(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<types::AssetDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_asset".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes an Asset from the given Parameter Context
    ///
    /// This endpoint will create a new Asset in the given Parameter Context. The Asset will be created with the given name and the contents of the file that is uploaded. The Asset will be created in the given Parameter Context, and will be available for use by any component that references the Parameter Context.
    ///
    /// Calls `DELETE /nifi-api/parameter-contexts/{contextId}/assets/{assetId}`.
    ///
    /// # Parameters
    /// - `asset_id`: The ID of the Asset
    /// - `disconnected_node_acknowledged`
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{parameterContextId}`.
    /// Requires `Write - /parameter-contexts/{parameterContextId}`.
    /// Requires `Read - for every component that is affected by the update`.
    /// Requires `Write - for every component that is affected by the update`.
    /// Requires `Read - for every currently inherited parameter context`.
    async fn delete_asset(
        &self,
        asset_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::AssetDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_asset".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the content of the asset with the given id
    ///
    /// Calls `GET /nifi-api/parameter-contexts/{contextId}/assets/{assetId}`.
    ///
    /// # Parameters
    /// - `asset_id`: The ID of the Asset
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{id}`.
    async fn get_asset_content(&self, asset_id: &str) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_asset_content".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Lists the assets that belong to the Parameter Context with the given ID
    ///
    /// Lists the assets that belong to the Parameter Context with the given ID.
    ///
    /// Calls `GET /nifi-api/parameter-contexts/{contextId}/assets`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{id}`.
    async fn get_assets(&self) -> Result<types::AssetsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_assets".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterContextsUpdateRequestsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsUpdateRequestsApi {
    /// Deletes the Update Request with the given ID
    ///
    /// Deletes the Update Request with the given ID. After a request is created via a POST to /nifi-api/parameter-contexts/update-requests, it is expected that the client will properly clean up the request by DELETE'ing it, once the Update process has completed. If the request is deleted before the request completes, then the Update request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/parameter-contexts/{contextId}/update-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Update Request
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
    async fn delete_update_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_update_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Update Request with the given ID
    ///
    /// Returns the Update Request with the given ID. Once an Update Request has been created by performing a POST to /nifi-api/parameter-contexts, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/parameter-contexts/{contextId}/update-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Update Request
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
    async fn get_parameter_context_update(
        &self,
        request_id: &str,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_context_update".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Initiate the Update Request of a Parameter Context
    ///
    /// This will initiate the process of updating a Parameter Context. Changing the value of a Parameter may require that one or more components be stopped and restarted, so this action may take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterContextUpdateRequestEntity, and the process of updating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-contexts/update-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-contexts/update-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-contexts/{contextId}/update-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{parameterContextId}`.
    /// Requires `Write - /parameter-contexts/{parameterContextId}`.
    /// Requires `Read - for every component that is affected by the update`.
    /// Requires `Write - for every component that is affected by the update`.
    /// Requires `Read - for every currently inherited parameter context`.
    /// Requires `Read - for any new inherited parameter context`.
    async fn submit_parameter_context_update(
        &self,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextUpdateRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_parameter_context_update".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ParameterContextsValidationRequestsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsValidationRequestsApi {
    /// Deletes the Validation Request with the given ID
    ///
    /// Deletes the Validation Request with the given ID. After a request is created via a POST to /nifi-api/validation-contexts, it is expected that the client will properly clean up the request by DELETE'ing it, once the validation process has completed. If the request is deleted before the request completes, then the Validation request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/parameter-contexts/{contextId}/validation-requests/{id}`.
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
    async fn delete_validation_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_validation_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Validation Request with the given ID
    ///
    /// Returns the Validation Request with the given ID. Once a Validation Request has been created by performing a POST to /nifi-api/validation-contexts, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/parameter-contexts/{contextId}/validation-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Validation Request
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
    async fn get_validation_request(
        &self,
        id: &str,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_validation_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Initiate a Validation Request to determine how the validity of components will change if a Parameter Context were to be updated
    ///
    /// This will initiate the process of validating all components whose Process Group is bound to the specified Parameter Context. Performing validation against an arbitrary number of components may be expect and take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterContextValidationRequestEntity, and the process of validating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-contexts/validation-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-contexts/validation-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-contexts/{contextId}/validation-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{parameterContextId}`.
    async fn submit_validation_request(
        &self,
        body: &types::ParameterContextValidationRequestEntity,
    ) -> Result<types::ParameterContextValidationRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "submit_validation_request".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The ParameterContexts API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ParameterContextsApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `context_id`: The ID of the Parameter Context
    fn assets<'b>(&'b self, context_id: &'b str) -> impl ParameterContextsAssetsApi + 'b;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `context_id`: The ID of the ParameterContext
    fn update_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> impl ParameterContextsUpdateRequestsApi + 'b;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `context_id`: The ID of the Parameter Context
    fn validation_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> impl ParameterContextsValidationRequestsApi + 'b;
    /// Create a Parameter Context
    ///
    /// Calls `POST /nifi-api/parameter-contexts`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /parameter-contexts`.
    /// Requires `Read - for every inherited parameter context`.
    async fn create_parameter_context(
        &self,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_parameter_context".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Deletes the Parameter Context with the given ID
    ///
    /// Deletes the Parameter Context with the given ID.
    ///
    /// Calls `DELETE /nifi-api/parameter-contexts/{id}`.
    ///
    /// # Parameters
    /// - `id`: The Parameter Context ID.
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
    /// Requires `Read - /parameter-contexts/{uuid}`.
    /// Requires `Write - /parameter-contexts/{uuid}`.
    /// Requires `Read - /process-groups/{uuid}, for any Process Group that is currently bound to the Parameter Context`.
    /// Requires `Write - /process-groups/{uuid}, for any Process Group that is currently bound to the Parameter Context`.
    async fn delete_parameter_context(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_parameter_context".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Parameter Context with the given ID
    ///
    /// Returns the Parameter Context with the given ID.
    ///
    /// Calls `GET /nifi-api/parameter-contexts/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Parameter Context
    /// - `include_inherited_parameters`: Whether or not to include inherited parameters from other parameter contexts, and therefore also overridden values.  If true, the result will be the 'effective' parameter context.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{id}`.
    async fn get_parameter_context(
        &self,
        id: &str,
        include_inherited_parameters: Option<bool>,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_context".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Modifies a Parameter Context
    ///
    /// This endpoint will update a Parameter Context to match the provided entity. However, this request will fail if any component is running and is referencing a Parameter in the Parameter Context. Generally, this endpoint is not called directly. Instead, an update request should be submitted by making a POST to the /parameter-contexts/update-requests endpoint. That endpoint will, in turn, call this endpoint.
    ///
    /// Calls `PUT /nifi-api/parameter-contexts/{id}`.
    ///
    /// # Parameters
    /// - `id`
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{id}`.
    /// Requires `Write - /parameter-contexts/{id}`.
    async fn update_parameter_context(
        &self,
        id: &str,
        body: &types::ParameterContextEntity,
    ) -> Result<types::ParameterContextEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_parameter_context".to_string(),
            version: "unknown".to_string(),
        })
    }
}
