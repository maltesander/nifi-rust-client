// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct ControllerApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerApi<'a> {
    /// Creates a new bulletin
    ///
    /// Calls `POST /nifi-api/controller/bulletin`.
    ///
    /// # Parameters
    /// - `body`: The reporting task configuration details.
    pub async fn create_bulletin(
        &self,
        body: &crate::v2_8_0::types::BulletinEntity,
    ) -> Result<crate::v2_8_0::types::BulletinEntity, NifiError> {
        self.client.post("/controller/bulletin", body).await
    }
    /// Gets the contents of the cluster
    ///
    /// Returns the contents of the cluster including all nodes and their status.
    ///
    /// Calls `GET /nifi-api/controller/cluster`.
    pub async fn get_cluster(&self) -> Result<crate::v2_8_0::types::ClusterDto, NifiError> {
        let e: crate::v2_8_0::types::ClusterEntity = self.client.get("/controller/cluster").await?;
        Ok(e.cluster)
    }
    /// Removes a node from the cluster
    ///
    /// Calls `DELETE /nifi-api/controller/cluster/nodes/{id}`.
    ///
    /// # Parameters
    /// - `id`: The node id.
    pub async fn delete_node(&self, id: &str) -> Result<crate::v2_8_0::types::NodeDto, NifiError> {
        let e: crate::v2_8_0::types::NodeEntity = self
            .client
            .delete_returning(&format!("/controller/cluster/nodes/{id}"))
            .await?;
        Ok(e.node)
    }
    /// Gets a node in the cluster
    ///
    /// Calls `GET /nifi-api/controller/cluster/nodes/{id}`.
    ///
    /// # Parameters
    /// - `id`: The node id.
    pub async fn get_node(&self, id: &str) -> Result<crate::v2_8_0::types::NodeDto, NifiError> {
        let e: crate::v2_8_0::types::NodeEntity = self
            .client
            .get(&format!("/controller/cluster/nodes/{id}"))
            .await?;
        Ok(e.node)
    }
    /// Updates a node in the cluster
    ///
    /// Calls `PUT /nifi-api/controller/cluster/nodes/{id}`.
    ///
    /// # Parameters
    /// - `id`: The node id.
    /// - `body`: The node configuration. The only configuration that will be honored at this endpoint is the status.
    pub async fn update_node(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::NodeEntity,
    ) -> Result<crate::v2_8_0::types::NodeDto, NifiError> {
        let e: crate::v2_8_0::types::NodeEntity = self
            .client
            .put(&format!("/controller/cluster/nodes/{id}"), body)
            .await?;
        Ok(e.node)
    }
    /// Retrieves the configuration for this NiFi Controller
    ///
    /// Calls `GET /nifi-api/controller/config`.
    pub async fn get_controller_config(
        &self,
    ) -> Result<crate::v2_8_0::types::ControllerConfigurationEntity, NifiError> {
        self.client.get("/controller/config").await
    }
    /// Retrieves the configuration for this NiFi
    ///
    /// Calls `PUT /nifi-api/controller/config`.
    ///
    /// # Parameters
    /// - `body`: The controller configuration.
    pub async fn update_controller_config(
        &self,
        body: &crate::v2_8_0::types::ControllerConfigurationEntity,
    ) -> Result<crate::v2_8_0::types::ControllerConfigurationEntity, NifiError> {
        self.client.put("/controller/config", body).await
    }
    /// Creates a new controller service
    ///
    /// Calls `POST /nifi-api/controller/controller-services`.
    ///
    /// # Parameters
    /// - `body`: The controller service configuration details.
    pub async fn create_controller_service(
        &self,
        body: &crate::v2_8_0::types::ControllerServiceEntity,
    ) -> Result<crate::v2_8_0::types::ControllerServiceEntity, NifiError> {
        self.client
            .post("/controller/controller-services", body)
            .await
    }
    /// Gets all flow analysis rules
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules`.
    pub async fn get_flow_analysis_rules(
        &self,
    ) -> Result<crate::v2_8_0::types::FlowAnalysisRulesEntity, NifiError> {
        self.client.get("/controller/flow-analysis-rules").await
    }
    /// Creates a new flow analysis rule
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules`.
    ///
    /// # Parameters
    /// - `body`: The flow analysis rule configuration details.
    pub async fn create_flow_analysis_rule(
        &self,
        body: &crate::v2_8_0::types::FlowAnalysisRuleEntity,
    ) -> Result<crate::v2_8_0::types::FlowAnalysisRuleEntity, NifiError> {
        self.client
            .post("/controller/flow-analysis-rules", body)
            .await
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
    pub async fn remove_flow_analysis_rule(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::FlowAnalysisRuleEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/controller/flow-analysis-rules/{id}"), &query)
            .await
    }
    /// Gets a flow analysis rule
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    pub async fn get_flow_analysis_rule(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::FlowAnalysisRuleEntity, NifiError> {
        self.client
            .get(&format!("/controller/flow-analysis-rules/{id}"))
            .await
    }
    /// Updates a flow analysis rule
    ///
    /// Calls `PUT /nifi-api/controller/flow-analysis-rules/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow analysis rule id.
    /// - `body`: The flow analysis rule configuration details.
    pub async fn update_flow_analysis_rule(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::FlowAnalysisRuleEntity,
    ) -> Result<crate::v2_8_0::types::FlowAnalysisRuleEntity, NifiError> {
        self.client
            .put(&format!("/controller/flow-analysis-rules/{id}"), body)
            .await
    }
    /// Purges history
    ///
    /// Calls `DELETE /nifi-api/controller/history`.
    ///
    /// # Parameters
    /// - `end_date`: Purge actions before this date/time.
    pub async fn delete_history(
        &self,
        end_date: &str,
    ) -> Result<crate::v2_8_0::types::HistoryDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("endDate", end_date.to_string()));
        let e: crate::v2_8_0::types::HistoryEntity = self
            .client
            .delete_returning_with_query("/controller/history", &query)
            .await?;
        Ok(e.history)
    }
    /// Retrieves summary information for installed NARs
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars`.
    pub async fn get_nar_summaries(
        &self,
    ) -> Result<crate::v2_8_0::types::NarSummariesEntity, NifiError> {
        self.client.get("/controller/nar-manager/nars").await
    }
    /// Uploads a NAR and requests for it to be installed
    ///
    /// Calls `POST /nifi-api/controller/nar-manager/nars/content`.
    pub async fn upload_nar(
        &self,
        filename: Option<&str>,
        data: Vec<u8>,
    ) -> Result<crate::v2_8_0::types::NarSummaryDto, NifiError> {
        let e: crate::v2_8_0::types::NarSummaryEntity = self
            .client
            .post_octet_stream("/controller/nar-manager/nars/content", filename, data)
            .await?;
        Ok(e.nar_summary)
    }
    /// Deletes an installed NAR
    ///
    /// Calls `DELETE /nifi-api/controller/nar-manager/nars/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the NAR.
    pub async fn delete_nar(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
        force: Option<bool>,
    ) -> Result<crate::v2_8_0::types::NarSummaryDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        if let Some(v) = force {
            query.push(("force", v.to_string()));
        }
        let e: crate::v2_8_0::types::NarSummaryEntity = self
            .client
            .delete_returning_with_query(&format!("/controller/nar-manager/nars/{id}"), &query)
            .await?;
        Ok(e.nar_summary)
    }
    /// Retrieves the summary information for the NAR with the given identifier
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars/{id}`.
    ///
    /// # Parameters
    /// - `id`: The id of the NAR.
    pub async fn get_nar_summary(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::NarDetailsEntity, NifiError> {
        self.client
            .get(&format!("/controller/nar-manager/nars/{id}"))
            .await
    }
    /// Creates a new parameter provider
    ///
    /// Calls `POST /nifi-api/controller/parameter-providers`.
    ///
    /// # Parameters
    /// - `body`: The parameter provider configuration details.
    pub async fn create_parameter_provider(
        &self,
        body: &crate::v2_8_0::types::ParameterProviderEntity,
    ) -> Result<crate::v2_8_0::types::ParameterProviderEntity, NifiError> {
        self.client
            .post("/controller/parameter-providers", body)
            .await
    }
    /// Gets the listing of available flow registry clients
    ///
    /// Calls `GET /nifi-api/controller/registry-clients`.
    pub async fn get_flow_registry_clients(
        &self,
    ) -> Result<crate::v2_8_0::types::FlowRegistryClientsEntity, NifiError> {
        self.client.get("/controller/registry-clients").await
    }
    /// Creates a new flow registry client
    ///
    /// Calls `POST /nifi-api/controller/registry-clients`.
    ///
    /// # Parameters
    /// - `body`: The flow registry client configuration details.
    pub async fn create_flow_registry_client(
        &self,
        body: &crate::v2_8_0::types::FlowRegistryClientEntity,
    ) -> Result<crate::v2_8_0::types::FlowRegistryClientEntity, NifiError> {
        self.client.post("/controller/registry-clients", body).await
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
    pub async fn delete_flow_registry_client(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::v2_8_0::types::FlowRegistryClientEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/controller/registry-clients/{id}"), &query)
            .await
    }
    /// Gets a flow registry client
    ///
    /// Calls `GET /nifi-api/controller/registry-clients/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow registry client id.
    pub async fn get_flow_registry_client(
        &self,
        id: &str,
    ) -> Result<crate::v2_8_0::types::FlowRegistryClientEntity, NifiError> {
        self.client
            .get(&format!("/controller/registry-clients/{id}"))
            .await
    }
    /// Updates a flow registry client
    ///
    /// Calls `PUT /nifi-api/controller/registry-clients/{id}`.
    ///
    /// # Parameters
    /// - `id`: The flow registry client id.
    /// - `body`: The flow registry client configuration details.
    pub async fn update_flow_registry_client(
        &self,
        id: &str,
        body: &crate::v2_8_0::types::FlowRegistryClientEntity,
    ) -> Result<crate::v2_8_0::types::FlowRegistryClientEntity, NifiError> {
        self.client
            .put(&format!("/controller/registry-clients/{id}"), body)
            .await
    }
    /// Retrieves the types of flow  that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/controller/registry-types`.
    pub async fn get_registry_client_types(
        &self,
    ) -> Result<crate::v2_8_0::types::FlowRegistryClientTypesEntity, NifiError> {
        self.client.get("/controller/registry-types").await
    }
    /// Creates a new reporting task
    ///
    /// Calls `POST /nifi-api/controller/reporting-tasks`.
    ///
    /// # Parameters
    /// - `body`: The reporting task configuration details.
    pub async fn create_reporting_task(
        &self,
        body: &crate::v2_8_0::types::ReportingTaskEntity,
    ) -> Result<crate::v2_8_0::types::ReportingTaskEntity, NifiError> {
        self.client.post("/controller/reporting-tasks", body).await
    }
    /// Imports a reporting task snapshot
    ///
    /// Calls `POST /nifi-api/controller/reporting-tasks/import`.
    ///
    /// # Parameters
    /// - `body`: The import request containing the reporting task snapshot to import.
    pub async fn import_reporting_task_snapshot(
        &self,
        body: &crate::v2_8_0::types::VersionedReportingTaskImportRequestEntity,
    ) -> Result<crate::v2_8_0::types::VersionedReportingTaskImportResponseEntity, NifiError> {
        self.client
            .post("/controller/reporting-tasks/import", body)
            .await
    }
    /// Gets status history for the node
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/controller/status/history`.
    pub async fn get_node_status_history(
        &self,
    ) -> Result<crate::v2_8_0::types::ComponentHistoryDto, NifiError> {
        let e: crate::v2_8_0::types::ComponentHistoryEntity =
            self.client.get("/controller/status/history").await?;
        Ok(e.component_history)
    }
    /// Scope operations to the `bulletins` sub-resource of a specific process group.
    ///
    /// - `id`: The flow analysis rule id.
    pub fn bulletins<'b>(&'b self, id: &'b str) -> ControllerBulletinsApi<'b> {
        ControllerBulletinsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `config` sub-resource of a specific process group.
    ///
    /// - `id`: The flow analysis rules id.
    pub fn config<'b>(&'b self, id: &'b str) -> ControllerConfigApi<'b> {
        ControllerConfigApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `content` sub-resource of a specific process group.
    ///
    /// - `id`: The id of the NAR.
    pub fn content<'b>(&'b self, id: &'b str) -> ControllerContentApi<'b> {
        ControllerContentApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `descriptors` sub-resource of a specific process group.
    ///
    /// - `id`: The flow analysis rule id.
    pub fn descriptors<'b>(&'b self, id: &'b str) -> ControllerDescriptorsApi<'b> {
        ControllerDescriptorsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `details` sub-resource of a specific process group.
    ///
    /// - `id`: The id of the NAR.
    pub fn details<'b>(&'b self, id: &'b str) -> ControllerDetailsApi<'b> {
        ControllerDetailsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `run_status` sub-resource of a specific process group.
    ///
    /// - `id`: The flow analysis rule id.
    pub fn run_status<'b>(&'b self, id: &'b str) -> ControllerRunStatusApi<'b> {
        ControllerRunStatusApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `state` sub-resource of a specific process group.
    ///
    /// - `id`: The flow analysis rule id.
    pub fn state<'b>(&'b self, id: &'b str) -> ControllerStateApi<'b> {
        ControllerStateApi {
            client: self.client,
            id,
        }
    }
}
pub struct ControllerBulletinsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerBulletinsApi<'a> {
    /// Clears bulletins for a flow analysis rule
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The request to clear bulletins.
    pub async fn clear_flow_analysis_rule_bulletins(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/controller/flow-analysis-rules/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
    /// Clears bulletins for a parameter provider
    ///
    /// Calls `POST /nifi-api/controller/parameter-providers/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The request to clear bulletins.
    pub async fn clear_parameter_provider_bulletins(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/controller/parameter-providers/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
    /// Clears bulletins for a registry client
    ///
    /// Calls `POST /nifi-api/controller/registry-clients/{id}/bulletins/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: The request to clear bulletins.
    pub async fn clear_registry_client_bulletins(
        &self,
        body: &crate::v2_8_0::types::ClearBulletinsRequestEntity,
    ) -> Result<crate::v2_8_0::types::ClearBulletinsResultEntity, NifiError> {
        let id = self.id;
        self.client
            .post(
                &format!("/controller/registry-clients/{id}/bulletins/clear-requests"),
                body,
            )
            .await
    }
}
pub struct ControllerConfigApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerConfigApi<'a> {
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `body`: The configuration analysis request.
    pub async fn analyze_flow_analysis_rule_configuration(
        &self,
        body: &crate::v2_8_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_8_0::types::ConfigurationAnalysisDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ConfigurationAnalysisEntity = self
            .client
            .post(
                &format!("/controller/flow-analysis-rules/{id}/config/analysis"),
                body,
            )
            .await?;
        Ok(e.configuration_analysis)
    }
    /// Performs verification of the Flow Analysis Rule's configuration
    ///
    /// This will initiate the process of verifying a given Flow Analysis Rule configuration. This may be a long-running task. As a result, this endpoint will immediately return a FlowAnalysisRuleConfigVerificationRequestEntity, and the process of performing the verification will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /flow-analysis-rules/{taskId}/verification-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /flow-analysis-rules/{serviceId}/verification-requests/{requestId}.
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `body`: The flow analysis rules configuration verification request.
    pub async fn submit_flow_analysis_rule_config_verification_request(
        &self,
        body: &crate::v2_8_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .post(
                &format!("/controller/flow-analysis-rules/{id}/config/verification-requests"),
                body,
            )
            .await?;
        Ok(e.request)
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/controller/flow-analysis-rules/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn delete_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .delete_returning(&format!(
                "/controller/flow-analysis-rules/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once an Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn get_flow_analysis_rule_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .get(&format!(
                "/controller/flow-analysis-rules/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
    /// Performs analysis of the component's configuration, providing information about which attributes are referenced.
    ///
    /// Calls `POST /nifi-api/controller/registry-clients/{id}/config/analysis`.
    ///
    /// # Parameters
    /// - `body`: The configuration analysis request.
    pub async fn analyze_flow_registry_client_configuration(
        &self,
        body: &crate::v2_8_0::types::ConfigurationAnalysisEntity,
    ) -> Result<crate::v2_8_0::types::ConfigurationAnalysisDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ConfigurationAnalysisEntity = self
            .client
            .post(
                &format!("/controller/registry-clients/{id}/config/analysis"),
                body,
            )
            .await?;
        Ok(e.configuration_analysis)
    }
    /// Performs verification of the Registry Client's configuration
    ///
    /// Initiates verification of a Registry Client configuration. The request returns immediately with a request entity while verification runs asynchronously. The client should poll /controller/registry-clients/{clientId}/config/verification-requests/{requestId} for status and DELETE the request once verification completes.
    ///
    /// Calls `POST /nifi-api/controller/registry-clients/{id}/config/verification-requests`.
    ///
    /// # Parameters
    /// - `body`: The registry client configuration verification request.
    pub async fn submit_registry_client_config_verification_request(
        &self,
        body: &crate::v2_8_0::types::VerifyConfigRequestEntity,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .post(
                &format!("/controller/registry-clients/{id}/config/verification-requests"),
                body,
            )
            .await?;
        Ok(e.request)
    }
    /// Deletes the Verification Request with the given ID
    ///
    /// Deletes the Verification Request with the given ID. After a request is created, it is expected that the client will properly clean up the request by DELETE'ing it, once the Verification process has completed. If the request is deleted before the request completes, then the Verification request will finish the step that it is currently performing and then will cancel any subsequent steps.
    ///
    /// Calls `DELETE /nifi-api/controller/registry-clients/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn delete_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .delete_returning(&format!(
                "/controller/registry-clients/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
    /// Returns the Verification Request with the given ID
    ///
    /// Returns the Verification Request with the given ID. Once a Verification Request has been created, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures.
    ///
    /// Calls `GET /nifi-api/controller/registry-clients/{id}/config/verification-requests/{requestId}`.
    ///
    /// # Parameters
    /// - `request_id`: The ID of the Verification Request
    pub async fn get_registry_client_verification_request(
        &self,
        request_id: &str,
    ) -> Result<crate::v2_8_0::types::VerifyConfigRequestDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::VerifyConfigRequestEntity = self
            .client
            .get(&format!(
                "/controller/registry-clients/{id}/config/verification-requests/{request_id}"
            ))
            .await?;
        Ok(e.request)
    }
}
pub struct ControllerContentApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerContentApi<'a> {
    /// Retrieves the content of the NAR with the given id
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars/{id}/content`.
    pub async fn download_nar(&self) -> Result<(), NifiError> {
        let id = self.id;
        self.client
            .get_void(&format!("/controller/nar-manager/nars/{id}/content"))
            .await
    }
}
pub struct ControllerDescriptorsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerDescriptorsApi<'a> {
    /// Gets a flow analysis rule property descriptor
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `property_name`: The property name.
    /// - `sensitive`: Property Descriptor requested sensitive status
    pub async fn get_flow_analysis_rule_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_8_0::types::PropertyDescriptorDto, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("propertyName", property_name.to_string()));
        if let Some(v) = sensitive {
            query.push(("sensitive", v.to_string()));
        }
        let e: crate::v2_8_0::types::PropertyDescriptorEntity = self
            .client
            .get_with_query(
                &format!("/controller/flow-analysis-rules/{id}/descriptors"),
                &query,
            )
            .await?;
        Ok(e.property_descriptor)
    }
    /// Gets a flow registry client property descriptor
    ///
    /// Calls `GET /nifi-api/controller/registry-clients/{id}/descriptors`.
    ///
    /// # Parameters
    /// - `property_name`: The property name.
    /// - `sensitive`: Property Descriptor requested sensitive status
    pub async fn get_property_descriptor(
        &self,
        property_name: &str,
        sensitive: Option<bool>,
    ) -> Result<crate::v2_8_0::types::PropertyDescriptorDto, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        query.push(("propertyName", property_name.to_string()));
        if let Some(v) = sensitive {
            query.push(("sensitive", v.to_string()));
        }
        let e: crate::v2_8_0::types::PropertyDescriptorEntity = self
            .client
            .get_with_query(
                &format!("/controller/registry-clients/{id}/descriptors"),
                &query,
            )
            .await?;
        Ok(e.property_descriptor)
    }
}
pub struct ControllerDetailsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerDetailsApi<'a> {
    /// Retrieves the component types available from the installed NARs
    ///
    /// Calls `GET /nifi-api/controller/nar-manager/nars/{id}/details`.
    pub async fn get_nar_details(
        &self,
    ) -> Result<crate::v2_8_0::types::NarDetailsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/controller/nar-manager/nars/{id}/details"))
            .await
    }
}
pub struct ControllerRunStatusApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerRunStatusApi<'a> {
    /// Updates run status of a flow analysis rule
    ///
    /// Calls `PUT /nifi-api/controller/flow-analysis-rules/{id}/run-status`.
    ///
    /// # Parameters
    /// - `body`: The flow analysis rule run status.
    pub async fn update_run_status(
        &self,
        body: &crate::v2_8_0::types::FlowAnalysisRuleRunStatusEntity,
    ) -> Result<crate::v2_8_0::types::FlowAnalysisRuleEntity, NifiError> {
        let id = self.id;
        self.client
            .put(
                &format!("/controller/flow-analysis-rules/{id}/run-status"),
                body,
            )
            .await
    }
}
pub struct ControllerStateApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ControllerStateApi<'a> {
    /// Gets the state for a flow analysis rule
    ///
    /// Calls `GET /nifi-api/controller/flow-analysis-rules/{id}/state`.
    pub async fn get_flow_analysis_rule_state(
        &self,
    ) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ComponentStateEntity = self
            .client
            .get(&format!("/controller/flow-analysis-rules/{id}/state"))
            .await?;
        Ok(e.component_state)
    }
    /// Clears the state for a flow analysis rule
    ///
    /// Calls `POST /nifi-api/controller/flow-analysis-rules/{id}/state/clear-requests`.
    ///
    /// # Parameters
    /// - `body`: Optional component state to perform a selective key removal. If omitted, clears all state.
    pub async fn clear_state(
        &self,
        body: &crate::v2_8_0::types::ComponentStateEntity,
    ) -> Result<crate::v2_8_0::types::ComponentStateDto, NifiError> {
        let id = self.id;
        let e: crate::v2_8_0::types::ComponentStateEntity = self
            .client
            .post(
                &format!("/controller/flow-analysis-rules/{id}/state/clear-requests"),
                body,
            )
            .await?;
        Ok(e.component_state)
    }
}
