// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ProcessorsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessorsApi<'a> {
    /// Submits a query to retrieve the run status details of all processors that are in the given list of Processor IDs
    ///
    /// Calls `POST /nifi-api/processors/run-status-details/queries`.
    ///
    /// # Parameters
    /// - `body`: The request for the processors that should be included in the results
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
    pub async fn get_processor_run_status_details(
        &self,
        body: &crate::v2_6_0::types::RunStatusDetailsRequestEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorsRunStatusDetailsEntity, NifiError> {
        self.client
            .post("/processors/run-status-details/queries", body)
            .await
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
    /// - `Write - /processors/{uuid}`
    /// - `Write - Parent Process Group - /process-groups/{uuid}`
    /// - `Read - any referenced Controller Services - /controller-services/{uuid}`
    pub async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/processors/{id}"), &query)
            .await
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
    pub async fn get_processor(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.client.get(&format!("/processors/{id}")).await
    }
    /// Updates a processor
    ///
    /// Calls `PUT /nifi-api/processors/{id}`.
    ///
    /// # Parameters
    /// - `id`: The processor id.
    /// - `body`: The processor configuration details.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// - `Write - /processors/{uuid}`
    /// - `Read - any referenced Controller Services if this request changes the reference - /controller-services/{uuid}`
    pub async fn update_processor(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ProcessorEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.client.put(&format!("/processors/{id}"), body).await
    }
    /// Scope operations to the `config` sub-resource of a specific process group.
    ///
    /// - `id`: The processor id.
    pub fn config<'b>(&'b self, id: &'b str) -> ProcessorsConfigApi<'b> {
        ProcessorsConfigApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `descriptors` sub-resource of a specific process group.
    ///
    /// - `id`: The processor id.
    pub fn descriptors<'b>(&'b self, id: &'b str) -> ProcessorsDescriptorsApi<'b> {
        ProcessorsDescriptorsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `diagnostics` sub-resource of a specific process group.
    ///
    /// - `id`: The processor id.
    pub fn diagnostics<'b>(&'b self, id: &'b str) -> ProcessorsDiagnosticsApi<'b> {
        ProcessorsDiagnosticsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `run_status` sub-resource of a specific process group.
    ///
    /// - `id`: The processor id.
    pub fn run_status<'b>(&'b self, id: &'b str) -> ProcessorsRunStatusApi<'b> {
        ProcessorsRunStatusApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `state` sub-resource of a specific process group.
    ///
    /// - `id`: The processor id.
    pub fn state<'b>(&'b self, id: &'b str) -> ProcessorsStateApi<'b> {
        ProcessorsStateApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `threads` sub-resource of a specific process group.
    ///
    /// - `id`: The processor id.
    pub fn threads<'b>(&'b self, id: &'b str) -> ProcessorsThreadsApi<'b> {
        ProcessorsThreadsApi {
            client: self.client,
            id,
        }
    }
}
pub struct ProcessorsConfigApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessorsConfigApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/processors/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `body`: The processor configuration analysis request.
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
    pub async fn analyze_configuration_2(
        &self,
        body: &crate::v2_6_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_6_0::types::ConfigurationAnalysisDto, NifiError> {
        let id = self.id;
        let e: crate::v2_6_0::types::ConfigurationAnalysisEntity = self
            .client
            .post(&format!("/processors/{id}/config/analysis"), body)
            .await?;
        Ok(e.configuration_analysis.unwrap_or_default())
    }
    /// Performs verification of the Processor's configuration
    ///
    /// This will initiate the process of verifying a given Processor configuration. This may be a long-running task. As a result, this endpoint will immediately return a ProcessorConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /processors/{processorId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /processors/{processorId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/processors/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `body`: The processor configuration verification request.
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
    pub async fn submit_processor_verification_request(
        &self,
        body: &crate::v2_6_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_6_0::types::VerifyConfigRequestEntity = self
            .client
            .post(
                &format!("/processors/{id}/config/verification-requests"),
                body,
            )
            .await?;
        Ok(e.request.unwrap_or_default())
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/processors/{id}/config/verification-requests/{requestId}`.
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
    pub async fn delete_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_6_0::types::VerifyConfigRequestEntity = self
            .client
            .delete_returning(&format!(
                "/processors/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request.unwrap_or_default())
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/processors/{id}/config/verification-requests/{requestId}`.
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
    pub async fn get_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_6_0::types::VerifyConfigRequestEntity = self
            .client
            .get(&format!(
                "/processors/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request.unwrap_or_default())
    }
}
pub struct ProcessorsDescriptorsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessorsDescriptorsApi<'a> {
    /// Gets the descriptor for a processor property
    ///
    /// Calls `GET /nifi-api/processors/{id}/descriptors`.
    ///
    /// # Parameters
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
    pub async fn get_property_descriptor_3(
        &self,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_6_0::types::PropertyDescriptorDto, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = client_id {
            query.push(("clientId", v.to_string()));
        }
        query.push(("propertyName", property_name.to_string()));
        if let Some(v) = sensitive {
            query.push(("sensitive", v.to_string()));
        }
        let e: crate::v2_6_0::types::PropertyDescriptorEntity = self
            .client
            .get_with_query(&format!("/processors/{id}/descriptors"), &query)
            .await?;
        Ok(e.property_descriptor.unwrap_or_default())
    }
}
pub struct ProcessorsDiagnosticsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessorsDiagnosticsApi<'a> {
    /// Gets diagnostics information about a processor
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/processors/{id}/diagnostics`.
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
    pub async fn get_processor_diagnostics(
        &self,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/processors/{id}/diagnostics"))
            .await
    }
}
pub struct ProcessorsRunStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessorsRunStatusApi<'a> {
    /// Updates run status of a processor
    ///
    /// Calls `PUT /nifi-api/processors/{id}/run-status`.
    ///
    /// # Parameters
    /// - `body`: The processor run status.
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
    pub async fn update_run_status_4(
        &self,
        body: &crate::v2_6_0::types::ProcessorRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/processors/{id}/run-status"), body)
            .await
    }
}
pub struct ProcessorsStateApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessorsStateApi<'a> {
    /// Gets the state for a processor
    ///
    /// Calls `GET /nifi-api/processors/{id}/state`.
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
    pub async fn get_state_2(&self) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_6_0::types::ComponentStateEntity =
            self.client.get(&format!("/processors/{id}/state")).await?;
        Ok(e.component_state.unwrap_or_default())
    }
    /// Clears the state for a processor
    ///
    /// Calls `POST /nifi-api/processors/{id}/state/clear-requests`.
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
    /// Requires `Write - /processors/{uuid}`.
    pub async fn clear_state_3(
        &self,
        body: &crate::v2_6_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_6_0::types::ComponentStateEntity = self
            .client
            .post(&format!("/processors/{id}/state/clear-requests"), body)
            .await?;
        Ok(e.component_state.unwrap_or_default())
    }
}
pub struct ProcessorsThreadsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessorsThreadsApi<'a> {
    /// Terminates a processor, essentially "deleting" its threads and any active tasks
    ///
    /// Calls `DELETE /nifi-api/processors/{id}/threads`.
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
    pub async fn terminate_processor(
        &self,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        let id = self.id;
        self.client
            .delete_returning(&format!("/processors/{id}/threads"))
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProcessorsApi for ProcessorsApi<'_> {
    type ProcessorsConfigApi<'b>
        = ProcessorsConfigApi<'b>
    where
        Self: 'b;
    fn config<'b>(&'b self, id: &'b str) -> Self::ProcessorsConfigApi<'b> {
        ProcessorsConfigApi {
            client: self.client,
            id,
        }
    }
    type ProcessorsDescriptorsApi<'b>
        = ProcessorsDescriptorsApi<'b>
    where
        Self: 'b;
    fn descriptors<'b>(&'b self, id: &'b str) -> Self::ProcessorsDescriptorsApi<'b> {
        ProcessorsDescriptorsApi {
            client: self.client,
            id,
        }
    }
    type ProcessorsDiagnosticsApi<'b>
        = ProcessorsDiagnosticsApi<'b>
    where
        Self: 'b;
    fn diagnostics<'b>(&'b self, id: &'b str) -> Self::ProcessorsDiagnosticsApi<'b> {
        ProcessorsDiagnosticsApi {
            client: self.client,
            id,
        }
    }
    type ProcessorsRunStatusApi<'b>
        = ProcessorsRunStatusApi<'b>
    where
        Self: 'b;
    fn run_status<'b>(&'b self, id: &'b str) -> Self::ProcessorsRunStatusApi<'b> {
        ProcessorsRunStatusApi {
            client: self.client,
            id,
        }
    }
    type ProcessorsStateApi<'b>
        = ProcessorsStateApi<'b>
    where
        Self: 'b;
    fn state<'b>(&'b self, id: &'b str) -> Self::ProcessorsStateApi<'b> {
        ProcessorsStateApi {
            client: self.client,
            id,
        }
    }
    type ProcessorsThreadsApi<'b>
        = ProcessorsThreadsApi<'b>
    where
        Self: 'b;
    fn threads<'b>(&'b self, id: &'b str) -> Self::ProcessorsThreadsApi<'b> {
        ProcessorsThreadsApi {
            client: self.client,
            id,
        }
    }
    async fn get_processor_run_status_details(
        &self,
        body: &crate::v2_6_0::types::RunStatusDetailsRequestEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorsRunStatusDetailsEntity, NifiError> {
        self.get_processor_run_status_details(body).await
    }
    async fn delete_processor(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.delete_processor(id, version, client_id, disconnected_node_acknowledged)
            .await
    }
    async fn get_processor(
        &self,
        id: &str,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.get_processor(id).await
    }
    async fn update_processor(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ProcessorEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.update_processor(id, body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProcessorsConfigApi for ProcessorsConfigApi<'_> {
    async fn analyze_configuration_2(
        &self,
        body: &crate::v2_6_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_6_0::types::ConfigurationAnalysisDto, NifiError> {
        self.analyze_configuration_2(body).await
    }
    async fn submit_processor_verification_request(
        &self,
        body: &crate::v2_6_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError> {
        self.submit_processor_verification_request(body).await
    }
    async fn delete_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError> {
        self.delete_verification_request_2(request_id).await
    }
    async fn get_verification_request_2(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_6_0::types::VerifyConfigRequestDto, NifiError> {
        self.get_verification_request_2(request_id).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProcessorsDescriptorsApi for ProcessorsDescriptorsApi<'_> {
    async fn get_property_descriptor_3(
        &self,
        client_id: Option<&str>,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_6_0::types::PropertyDescriptorDto, NifiError> {
        self.get_property_descriptor_3(client_id, property_name, sensitive)
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProcessorsDiagnosticsApi for ProcessorsDiagnosticsApi<'_> {
    async fn get_processor_diagnostics(
        &self,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.get_processor_diagnostics().await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProcessorsRunStatusApi for ProcessorsRunStatusApi<'_> {
    async fn update_run_status_4(
        &self,
        body: &crate::v2_6_0::types::ProcessorRunStatusEntity,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.update_run_status_4(body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProcessorsStateApi for ProcessorsStateApi<'_> {
    async fn get_state_2(&self) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError> {
        self.get_state_2().await
    }
    async fn clear_state_3(
        &self,
        body: &crate::v2_6_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_6_0::types::ComponentStateDto, NifiError> {
        self.clear_state_3(body).await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_6_0::traits::ProcessorsThreadsApi for ProcessorsThreadsApi<'_> {
    async fn terminate_processor(
        &self,
    ) -> Result<crate::v2_6_0::types::ProcessorEntity, NifiError> {
        self.terminate_processor().await
    }
}
