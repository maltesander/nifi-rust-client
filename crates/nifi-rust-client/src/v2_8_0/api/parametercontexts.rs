// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ParameterContextsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterContextsApi<'a> {
    /// Create a Parameter Context
    ///
    /// Calls `POST /nifi-api/parameter-contexts`.
    ///
    /// # Parameters
    /// - `body`: The Parameter Context.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Write - /parameter-contexts`
    /// - `Read - for every inherited parameter context`
    pub async fn create_parameter_context(
        &self,
        body: &crate::v2_8_0::types::ParameterContextEntity,
    ) -> Result<crate::v2_8_0::types::ParameterContextEntity, NifiError> {
        self.client.post("/parameter-contexts", body).await
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
    /// - `Read - /parameter-contexts/{uuid}`
    /// - `Write - /parameter-contexts/{uuid}`
    /// - `Read - /process-groups/{uuid}, for any Process Group that is currently bound to the Parameter Context`
    /// - `Write - /process-groups/{uuid}, for any Process Group that is currently bound to the Parameter Context`
    pub async fn delete_parameter_context(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ParameterContextEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/parameter-contexts/{id}"), &query)
            .await
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
    pub async fn get_parameter_context(
        &self,
        id: &str,
        include_inherited_parameters: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ParameterContextEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = include_inherited_parameters {
            query.push(("includeInheritedParameters", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/parameter-contexts/{id}"), &query)
            .await
    }
    /// Modifies a Parameter Context
    ///
    /// This endpoint will update a Parameter Context to match the provided entity. However, this request will fail if any component is running and is referencing a Parameter in the Parameter Context. Generally, this endpoint is not called directly. Instead, an update request should be submitted by making a POST to the /parameter-contexts/update-requests endpoint. That endpoint will, in turn, call this endpoint.
    ///
    /// Calls `PUT /nifi-api/parameter-contexts/{id}`.
    ///
    /// # Parameters
    /// - `body`: The updated Parameter Context
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /parameter-contexts/{id}`
    /// - `Write - /parameter-contexts/{id}`
    pub async fn update_parameter_context(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::ParameterContextEntity,
    ) -> Result<crate::v2_8_0::types::ParameterContextEntity, NifiError> {
        self.client
            .put(&format!("/parameter-contexts/{id}"), body)
            .await
    }
    /// Scope operations to the `assets` sub-resource of a specific process group.
    ///
    /// - `context_id`: The ID of the Parameter Context
    pub fn assets<'b>(&'b self, context_id: &'b str) -> ParameterContextsAssetsApi<'b> {
        ParameterContextsAssetsApi {
            client: self.client,
            context_id,
        }
    }
    /// Scope operations to the `update_requests` sub-resource of a specific process group.
    ///
    /// - `context_id`: The ID of the ParameterContext
    pub fn update_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> ParameterContextsUpdateRequestsApi<'b> {
        ParameterContextsUpdateRequestsApi {
            client: self.client,
            context_id,
        }
    }
    /// Scope operations to the `validation_requests` sub-resource of a specific process group.
    ///
    /// - `context_id`: The ID of the Parameter Context
    pub fn validation_requests<'b>(
        &'b self,
        context_id: &'b str,
    ) -> ParameterContextsValidationRequestsApi<'b> {
        ParameterContextsValidationRequestsApi {
            client: self.client,
            context_id,
        }
    }
}
pub struct ParameterContextsAssetsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) context_id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterContextsAssetsApi<'a> {
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
    pub async fn get_assets(&self) -> Result<crate::v2_8_0::types::AssetsEntity, NifiError> {
        let context_id = self.context_id;
        self.client
            .get(&format!("/parameter-contexts/{context_id}/assets"))
            .await
    }
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
    /// - `Read - /parameter-contexts/{parameterContextId}`
    /// - `Write - /parameter-contexts/{parameterContextId}`
    /// - `Read - for every component that is affected by the update`
    /// - `Write - for every component that is affected by the update`
    /// - `Read - for every currently inherited parameter context`
    pub async fn create_asset(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<crate::v2_8_0::types::AssetDto, NifiError> {
        let context_id = self.context_id;
        let e: crate::v2_8_0::types::AssetEntity = self
            .client
            .post_octet_stream(
                &format!("/parameter-contexts/{context_id}/assets"),
                filename,
                data,
            )
            .await?;
        Ok(e.asset.unwrap_or_default())
    }
    /// Deletes an Asset from the given Parameter Context
    ///
    /// This endpoint will create a new Asset in the given Parameter Context. The Asset will be created with the given name and the contents of the file that is uploaded. The Asset will be created in the given Parameter Context, and will be available for use by any component that references the Parameter Context.
    ///
    /// Calls `DELETE /nifi-api/parameter-contexts/{contextId}/assets/{assetId}`.
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
    /// - `Read - /parameter-contexts/{parameterContextId}`
    /// - `Write - /parameter-contexts/{parameterContextId}`
    /// - `Read - for every component that is affected by the update`
    /// - `Write - for every component that is affected by the update`
    /// - `Read - for every currently inherited parameter context`
    pub async fn delete_asset(
        &self,
        asset_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::AssetDto, NifiError> {
        let context_id = self.context_id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        let e: crate::v2_8_0::types::AssetEntity = self
            .client
            .delete_returning_with_query(
                &format!("/parameter-contexts/{context_id}/assets/{asset_id}"),
                &query,
            )
            .await?;
        Ok(e.asset.unwrap_or_default())
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
    pub async fn get_asset_content(&self, asset_id: &str) -> Result<(), NifiError> {
        let context_id = self.context_id;
        self.client
            .get_void(&format!(
                "/parameter-contexts/{context_id}/assets/{asset_id}"
            ))
            .await
    }
}
pub struct ParameterContextsUpdateRequestsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) context_id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterContextsUpdateRequestsApi<'a> {
    /// Initiate the Update Request of a Parameter Context
    ///
    /// This will initiate the process of updating a Parameter Context. Changing the value of a Parameter may require that one or more components be stopped and restarted, so this action may take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterContextUpdateRequestEntity, and the process of updating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-contexts/update-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-contexts/update-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-contexts/{contextId}/update-requests`.
    ///
    /// # Parameters
    /// - `body`: The updated version of the parameter context.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Read - /parameter-contexts/{parameterContextId}`
    /// - `Write - /parameter-contexts/{parameterContextId}`
    /// - `Read - for every component that is affected by the update`
    /// - `Write - for every component that is affected by the update`
    /// - `Read - for every currently inherited parameter context`
    /// - `Read - for any new inherited parameter context`
    pub async fn submit_parameter_context_update(
        &self,
        body: &crate::v2_8_0::types::ParameterContextEntity,
    ) -> Result<crate::v2_8_0::types::ParameterContextUpdateRequestEntity, NifiError> {
        let context_id = self.context_id;
        self.client
            .post(
                &format!("/parameter-contexts/{context_id}/update-requests"),
                body,
            )
            .await
    }
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
    pub async fn delete_update_request(
        &self,
        request_id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ParameterContextUpdateRequestEntity, NifiError> {
        let context_id = self.context_id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(
                &format!("/parameter-contexts/{context_id}/update-requests/{request_id}"),
                &query,
            )
            .await
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
    pub async fn get_parameter_context_update(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::ParameterContextUpdateRequestEntity, NifiError> {
        let context_id = self.context_id;
        self.client
            .get(&format!(
                "/parameter-contexts/{context_id}/update-requests/{request_id}"
            ))
            .await
    }
}
pub struct ParameterContextsValidationRequestsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) context_id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ParameterContextsValidationRequestsApi<'a> {
    /// Initiate a Validation Request to determine how the validity of components will change if a Parameter Context were to be updated
    ///
    /// This will initiate the process of validating all components whose Process Group is bound to the specified Parameter Context. Performing validation against an arbitrary number of components may be expect and take significantly more time than many other REST API actions. As a result, this endpoint will immediately return a ParameterContextValidationRequestEntity, and the process of validating the necessary components will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /parameter-contexts/validation-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /parameter-contexts/validation-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/parameter-contexts/{contextId}/validation-requests`.
    ///
    /// # Parameters
    /// - `body`: The validation request
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
    pub async fn submit_validation_request(
        &self,
        body: &crate::v2_8_0::types::ParameterContextValidationRequestEntity,
    ) -> Result<crate::v2_8_0::types::ParameterContextValidationRequestEntity, NifiError> {
        let context_id = self.context_id;
        self.client
            .post(
                &format!("/parameter-contexts/{context_id}/validation-requests"),
                body,
            )
            .await
    }
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
    pub async fn delete_validation_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::ParameterContextValidationRequestEntity, NifiError> {
        let context_id = self.context_id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(
                &format!("/parameter-contexts/{context_id}/validation-requests/{id}"),
                &query,
            )
            .await
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
    pub async fn get_validation_request(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::ParameterContextValidationRequestEntity, NifiError> {
        let context_id = self.context_id;
        self.client
            .get(&format!(
                "/parameter-contexts/{context_id}/validation-requests/{id}"
            ))
            .await
    }
}
