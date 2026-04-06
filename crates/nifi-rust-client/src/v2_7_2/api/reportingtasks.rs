use crate::NifiClient;
use crate::NifiError;
pub struct ReportingTasksApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ReportingTasksApi<'a> {
    /// Deletes a reporting task
    ///
    /// Calls `DELETE /nifi-api/reporting-tasks/{id}`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_reporting_task(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_7_2::types::ReportingTaskEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/reporting-tasks/{id}"), &query)
            .await
    }
    /// Gets a reporting task
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    pub async fn get_reporting_task(
        &self,
        id: &str,
    ) -> Result<crate::v2_7_2::types::ReportingTaskEntity, NifiError> {
        self.client.get(&format!("/reporting-tasks/{id}")).await
    }
    /// Updates a reporting task
    ///
    /// Calls `PUT /nifi-api/reporting-tasks/{id}`.
    ///
    /// # Parameters
    /// - `id`: The reporting task id.
    /// - `body`: The reporting task configuration details.
    pub async fn update_reporting_task(
        &self,
        id: &str,
        body: &crate::v2_7_2::types::ReportingTaskEntity,
    ) -> Result<crate::v2_7_2::types::ReportingTaskEntity, NifiError> {
        self.client
            .put(&format!("/reporting-tasks/{id}"), body)
            .await
    }
    /// Scope operations to the `bulletins` sub-resource of a specific process group.
    ///
    /// - `id`: The reporting task id.
    pub fn bulletins<'b>(&'b self, id: &'b str) -> ReportingTasksBulletinsApi<'b> {
        ReportingTasksBulletinsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `config` sub-resource of a specific process group.
    ///
    /// - `id`: The reporting task id.
    pub fn config<'b>(&'b self, id: &'b str) -> ReportingTasksConfigApi<'b> {
        ReportingTasksConfigApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `descriptors` sub-resource of a specific process group.
    ///
    /// - `id`: The reporting task id.
    pub fn descriptors<'b>(&'b self, id: &'b str) -> ReportingTasksDescriptorsApi<'b> {
        ReportingTasksDescriptorsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `run_status` sub-resource of a specific process group.
    ///
    /// - `id`: The reporting task id.
    pub fn run_status<'b>(&'b self, id: &'b str) -> ReportingTasksRunStatusApi<'b> {
        ReportingTasksRunStatusApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `state` sub-resource of a specific process group.
    ///
    /// - `id`: The reporting task id.
    pub fn state<'b>(&'b self, id: &'b str) -> ReportingTasksStateApi<'b> {
        ReportingTasksStateApi {
            client: self.client,
            id,
        }
    }
}
pub struct ReportingTasksBulletinsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ReportingTasksBulletinsApi<'a> {
    /// Clears bulletins for a reporting task
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The clear bulletin request specifying the timestamp from which to clear bulletins.
    pub async fn clear_bulletins_7(
        &self,
        body: &crate::v2_7_2::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_7_2::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/reporting-tasks/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
}
pub struct ReportingTasksConfigApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ReportingTasksConfigApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `body`: The configuration analysis request.
    pub async fn analyze_configuration_3(
        &self,
        body: &crate::v2_7_2::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_7_2::types::ConfigurationAnalysisDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::ConfigurationAnalysisEntity = self
            .client
            .post(&format!("/reporting-tasks/{id}/config/analysis"), body)
            .await?;
        Ok(e.configuration_analysis)
    }
    /// Performs verification of the Reporting Task's configuration
    ///
    /// This will initiate the process of verifying a given Reporting Task configuration. This may be a long-running task. As a result, this endpoint will immediately return a ReportingTaskConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /reporting-tasks/{taskId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /reporting-tasks/{serviceId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `body`: The reporting task configuration verification request.
    pub async fn submit_config_verification_request_2(
        &self,
        body: &crate::v2_7_2::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::VerifyConfigRequestEntity = self
            .client
            .post(
                &format!("/reporting-tasks/{id}/config/verification-requests"),
                body,
            )
            .await?;
        Ok(e.request)
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/reporting-tasks/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn delete_verification_request_3(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::VerifyConfigRequestEntity = self
            .client
            .delete_returning(&format!(
                "/reporting-tasks/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn get_verification_request_3(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_7_2::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::VerifyConfigRequestEntity = self
            .client
            .get(&format!(
                "/reporting-tasks/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
}
pub struct ReportingTasksDescriptorsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ReportingTasksDescriptorsApi<'a> {
    /// Gets a reporting task property descriptor
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `property_name`: The property name.
    /// - `sensitive`: Property Descriptor requested sensitive status
    pub async fn get_property_descriptor_4(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_7_2::types::PropertyDescriptorDto, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("propertyName", property_name.to_string()));
        if let Some(v) = sensitive {
            query.push(("sensitive", v.to_string()));
        }
        let e: crate::v2_7_2::types::PropertyDescriptorEntity = self
            .client
            .get_with_query(&format!("/reporting-tasks/{id}/descriptors"), &query)
            .await?;
        Ok(e.property_descriptor)
    }
}
pub struct ReportingTasksRunStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ReportingTasksRunStatusApi<'a> {
    /// Updates run status of a reporting task
    ///
    /// Calls `PUT /nifi-api/reporting-tasks/{id}/run-status`.
    ///
    /// # Parameters
    /// - `body`: The reporting task run status.
    pub async fn update_run_status_5(
        &self,
        body: &crate::v2_7_2::types::ReportingTaskRunStatusEntity,
    ) -> Result<crate::v2_7_2::types::ReportingTaskEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/reporting-tasks/{id}/run-status"), body)
            .await
    }
}
pub struct ReportingTasksStateApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ReportingTasksStateApi<'a> {
    /// Gets the state for a reporting task
    ///
    /// Calls `GET /nifi-api/reporting-tasks/{id}/state`.
    pub async fn get_state_4(&self) -> Result<crate::v2_7_2::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::ComponentStateEntity = self
            .client
            .get(&format!("/reporting-tasks/{id}/state"))
            .await?;
        Ok(e.component_state)
    }
    /// Clears the state for a reporting task
    ///
    /// Calls `POST /nifi-api/reporting-tasks/{id}/state/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: Optional component state to perform a selective key removal. If omitted, clears all state.
    pub async fn clear_state_4(
        &self,
        body: &crate::v2_7_2::types::ComponentStateEntity,
    ) -> Result<crate::v2_7_2::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_7_2::types::ComponentStateEntity = self
            .client
            .post(&format!("/reporting-tasks/{id}/state/clear-requests"), body)
            .await?;
        Ok(e.component_state)
    }
}
