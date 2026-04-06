use crate::NifiClient;
use crate::NifiError;
pub struct ProcessGroupsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsApi<'a> {
    /// Deletes the Replace Request with the given ID
    ///
    /// Deletes the Replace Request with the given ID. After a request is created via a POST to /process-groups/{id}/replace-requests, it is expected that the client will properly clean up the request by DELETE'ing it, once the Replace process has completed. If the request is deleted before the request completes, then the Replace request will finish the step that it is currently performing and then will cancel any subsequent steps. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/process-groups/replace-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Update Request
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn delete_replace_process_group_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::types::ProcessGroupReplaceRequestEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = disconnected_node_acknowledged {
            query.push(("disconnectedNodeAcknowledged", v.to_string()));
        }
        self.client
            .delete_returning_with_query(&format!("/process-groups/replace-requests/{id}"), &query)
            .await
    }
    /// Returns the Replace Request with the given ID
    ///
    /// Returns the Replace Request with the given ID. Once a Replace Request has been created by performing a POST to /process-groups/{id}/replace-requests, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/process-groups/replace-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Replace Request
    pub async fn get_replace_process_group_request(
        &self,
        id: &str,
    ) -> Result<crate::types::ProcessGroupReplaceRequestEntity, NifiError> {
        self.client
            .get(&format!("/process-groups/replace-requests/{id}"))
            .await
    }
    /// Deletes a process group
    ///
    /// Calls `DELETE /nifi-api/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `version`: The revision is used to verify the client is working with the latest version of the flow.
    /// - `client_id`: If the client id is not specified, new one will be generated. This value (whether specified or generated) is included in the response.
    /// - `disconnected_node_acknowledged`: Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub async fn remove_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<crate::types::ProcessGroupEntity, NifiError> {
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
            .delete_returning_with_query(&format!("/process-groups/{id}"), &query)
            .await
    }
    /// Gets a process group
    ///
    /// Calls `GET /nifi-api/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    pub async fn get_process_group(
        &self,
        id: &str,
    ) -> Result<crate::types::ProcessGroupEntity, NifiError> {
        self.client.get(&format!("/process-groups/{id}")).await
    }
    /// Updates a process group
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `body`: The process group configuration details.
    pub async fn update_process_group(
        &self,
        id: &str,
        body: &crate::types::ProcessGroupEntity,
    ) -> Result<crate::types::ProcessGroupEntity, NifiError> {
        self.client
            .put(&format!("/process-groups/{id}"), body)
            .await
    }
    /// Scope operations to the `connections` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn connections<'b>(&'b self, id: &'b str) -> ProcessGroupsConnectionsApi<'b> {
        ProcessGroupsConnectionsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `controller_services` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn controller_services<'b>(
        &'b self,
        id: &'b str,
    ) -> ProcessGroupsControllerServicesApi<'b> {
        ProcessGroupsControllerServicesApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `copy` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn copy<'b>(&'b self, id: &'b str) -> ProcessGroupsCopyApi<'b> {
        ProcessGroupsCopyApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `download` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn download<'b>(&'b self, id: &'b str) -> ProcessGroupsDownloadApi<'b> {
        ProcessGroupsDownloadApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `empty_all_connections_requests` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn empty_all_connections_requests<'b>(
        &'b self,
        id: &'b str,
    ) -> ProcessGroupsEmptyAllConnectionsRequestsApi<'b> {
        ProcessGroupsEmptyAllConnectionsRequestsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `flow_contents` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn flow_contents<'b>(&'b self, id: &'b str) -> ProcessGroupsFlowContentsApi<'b> {
        ProcessGroupsFlowContentsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `funnels` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn funnels<'b>(&'b self, id: &'b str) -> ProcessGroupsFunnelsApi<'b> {
        ProcessGroupsFunnelsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `input_ports` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn input_ports<'b>(&'b self, id: &'b str) -> ProcessGroupsInputPortsApi<'b> {
        ProcessGroupsInputPortsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `labels` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn labels<'b>(&'b self, id: &'b str) -> ProcessGroupsLabelsApi<'b> {
        ProcessGroupsLabelsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `local_modifications` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn local_modifications<'b>(
        &'b self,
        id: &'b str,
    ) -> ProcessGroupsLocalModificationsApi<'b> {
        ProcessGroupsLocalModificationsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `output_ports` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn output_ports<'b>(&'b self, id: &'b str) -> ProcessGroupsOutputPortsApi<'b> {
        ProcessGroupsOutputPortsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `paste` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn paste<'b>(&'b self, id: &'b str) -> ProcessGroupsPasteApi<'b> {
        ProcessGroupsPasteApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `process_groups` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn process_groups<'b>(&'b self, id: &'b str) -> ProcessGroupsProcessGroupsApi<'b> {
        ProcessGroupsProcessGroupsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `processors` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn processors<'b>(&'b self, id: &'b str) -> ProcessGroupsProcessorsApi<'b> {
        ProcessGroupsProcessorsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `remote_process_groups` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn remote_process_groups<'b>(
        &'b self,
        id: &'b str,
    ) -> ProcessGroupsRemoteProcessGroupsApi<'b> {
        ProcessGroupsRemoteProcessGroupsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `replace_requests` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn replace_requests<'b>(&'b self, id: &'b str) -> ProcessGroupsReplaceRequestsApi<'b> {
        ProcessGroupsReplaceRequestsApi {
            client: self.client,
            id,
        }
    }
    /// Scope operations to the `snippet_instance` sub-resource of a specific process group.
    ///
    /// - `id`: The process group id.
    pub fn snippet_instance<'b>(&'b self, id: &'b str) -> ProcessGroupsSnippetInstanceApi<'b> {
        ProcessGroupsSnippetInstanceApi {
            client: self.client,
            id,
        }
    }
}
pub struct ProcessGroupsConnectionsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsConnectionsApi<'a> {
    /// Gets all connections
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/connections`.
    pub async fn get_connections(&self) -> Result<crate::types::ConnectionsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/connections"))
            .await
    }
    /// Creates a connection
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/connections`.
    ///
    /// # Parameters
    /// - `body`: The connection configuration details.
    pub async fn create_connection(
        &self,
        body: &crate::types::ConnectionEntity,
    ) -> Result<crate::types::ConnectionEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/connections"), body)
            .await
    }
}
pub struct ProcessGroupsControllerServicesApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsControllerServicesApi<'a> {
    /// Creates a new controller service
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/controller-services`.
    ///
    /// # Parameters
    /// - `body`: The controller service configuration details.
    pub async fn create_controller_service_1(
        &self,
        body: &crate::types::ControllerServiceEntity,
    ) -> Result<crate::types::ControllerServiceEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/controller-services"), body)
            .await
    }
}
pub struct ProcessGroupsCopyApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsCopyApi<'a> {
    /// Generates a copy response for the given copy request
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/copy`.
    ///
    /// # Parameters
    /// - `body`: The request including the components to be copied from the specified Process Group.
    pub async fn copy(
        &self,
        body: &crate::types::CopyRequestEntity,
    ) -> Result<crate::types::CopyResponseEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/copy"), body)
            .await
    }
}
pub struct ProcessGroupsDownloadApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsDownloadApi<'a> {
    /// Gets a process group for download
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/download`.
    ///
    /// # Parameters
    /// - `include_referenced_services`: If referenced services from outside the target group should be included
    pub async fn export_process_group(
        &self,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = include_referenced_services {
            query.push(("includeReferencedServices", v.to_string()));
        }
        self.client
            .get_void_with_query(&format!("/process-groups/{id}/download"), &query)
            .await
    }
}
pub struct ProcessGroupsEmptyAllConnectionsRequestsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsEmptyAllConnectionsRequestsApi<'a> {
    /// Creates a request to drop all flowfiles of all connection queues in this process group.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/empty-all-connections-requests`.
    pub async fn create_empty_all_connections_request(&self) -> Result<(), NifiError> {
        let id = self.id;
        self.client
            .post_void_no_body(&format!(
                "/process-groups/{id}/empty-all-connections-requests"
            ))
            .await
    }
    /// Cancels and/or removes a request to drop all flowfiles.
    ///
    /// Calls `DELETE /nifi-api/process-groups/{id}/empty-all-connections-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `drop_request_id`: The drop request id.
    pub async fn remove_drop_request_1(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::types::DropRequestDto, NifiError> {
        let id = self.id;
        let e: crate::types::DropRequestEntity = self
            .client
            .delete_returning(&format!(
                "/process-groups/{id}/empty-all-connections-requests/{drop_request_id}"
            ))
            .await?;
        Ok(e.drop_request)
    }
    /// Gets the current status of a drop all flowfiles request.
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/empty-all-connections-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `drop_request_id`: The drop request id.
    pub async fn get_drop_all_flowfiles_request(
        &self,
        drop_request_id: &str,
    ) -> Result<crate::types::DropRequestDto, NifiError> {
        let id = self.id;
        let e: crate::types::DropRequestEntity = self
            .client
            .get(&format!(
                "/process-groups/{id}/empty-all-connections-requests/{drop_request_id}"
            ))
            .await?;
        Ok(e.drop_request)
    }
}
pub struct ProcessGroupsFlowContentsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsFlowContentsApi<'a> {
    /// Replace Process Group contents with the given ID with the specified Process Group contents
    ///
    /// This endpoint is used for replication within a cluster, when replacing a flow with a new flow. It expects that the flow beingreplaced is not under version control and that the given snapshot will not modify any Processor that is currently running or any Controller Service that is enabled. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}/flow-contents`.
    ///
    /// # Parameters
    /// - `body`: The process group replace request entity.
    pub async fn replace_process_group(
        &self,
        body: &crate::types::ProcessGroupImportEntity,
    ) -> Result<crate::types::ProcessGroupImportEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/process-groups/{id}/flow-contents"), body)
            .await
    }
}
pub struct ProcessGroupsFunnelsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsFunnelsApi<'a> {
    /// Gets all funnels
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/funnels`.
    pub async fn get_funnels(&self) -> Result<crate::types::FunnelsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/funnels"))
            .await
    }
    /// Creates a funnel
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/funnels`.
    ///
    /// # Parameters
    /// - `body`: The funnel configuration details.
    pub async fn create_funnel(
        &self,
        body: &crate::types::FunnelEntity,
    ) -> Result<crate::types::FunnelEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/funnels"), body)
            .await
    }
}
pub struct ProcessGroupsInputPortsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsInputPortsApi<'a> {
    /// Gets all input ports
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/input-ports`.
    pub async fn get_input_ports(&self) -> Result<crate::types::InputPortsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/input-ports"))
            .await
    }
    /// Creates an input port
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/input-ports`.
    ///
    /// # Parameters
    /// - `body`: The input port configuration details.
    pub async fn create_input_port(
        &self,
        body: &crate::types::PortEntity,
    ) -> Result<crate::types::PortEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/input-ports"), body)
            .await
    }
}
pub struct ProcessGroupsLabelsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsLabelsApi<'a> {
    /// Gets all labels
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/labels`.
    pub async fn get_labels(&self) -> Result<crate::types::LabelsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/labels"))
            .await
    }
    /// Creates a label
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/labels`.
    ///
    /// # Parameters
    /// - `body`: The label configuration details.
    pub async fn create_label(
        &self,
        body: &crate::types::LabelEntity,
    ) -> Result<crate::types::LabelEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/labels"), body)
            .await
    }
}
pub struct ProcessGroupsLocalModificationsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsLocalModificationsApi<'a> {
    /// Gets a list of local modifications to the Process Group since it was last synchronized with the Flow Registry
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/local-modifications`.
    pub async fn get_local_modifications(
        &self,
    ) -> Result<crate::types::FlowComparisonEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/local-modifications"))
            .await
    }
}
pub struct ProcessGroupsOutputPortsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsOutputPortsApi<'a> {
    /// Gets all output ports
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/output-ports`.
    pub async fn get_output_ports(&self) -> Result<crate::types::OutputPortsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/output-ports"))
            .await
    }
    /// Creates an output port
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/output-ports`.
    ///
    /// # Parameters
    /// - `body`: The output port configuration.
    pub async fn create_output_port(
        &self,
        body: &crate::types::PortEntity,
    ) -> Result<crate::types::PortEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/output-ports"), body)
            .await
    }
}
pub struct ProcessGroupsPasteApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsPasteApi<'a> {
    /// Pastes into the specified process group
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}/paste`.
    ///
    /// # Parameters
    /// - `body`: The request including the components to be pasted into the specified Process Group.
    pub async fn paste(
        &self,
        body: &crate::types::PasteRequestEntity,
    ) -> Result<crate::types::PasteResponseEntity, NifiError> {
        let id = self.id;
        self.client
            .put(&format!("/process-groups/{id}/paste"), body)
            .await
    }
}
pub struct ProcessGroupsProcessGroupsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsProcessGroupsApi<'a> {
    /// Gets all process groups
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/process-groups`.
    pub async fn get_process_groups(&self) -> Result<crate::types::ProcessGroupsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/process-groups"))
            .await
    }
    /// Creates a process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups`.
    ///
    /// # Parameters
    /// - `parameter_context_handling_strategy`: Handling Strategy controls whether to keep or replace Parameter Contexts
    /// - `body`: The process group configuration details.
    pub async fn create_process_group(
        &self,
        parameter_context_handling_strategy: Option<crate::types::ParameterContextHandlingStrategy>,
        body: &crate::types::ProcessGroupEntity,
    ) -> Result<crate::types::ProcessGroupEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = parameter_context_handling_strategy {
            query.push(("parameterContextHandlingStrategy", v.to_string()));
        }
        self.client
            .post_with_query(
                &format!("/process-groups/{id}/process-groups"),
                body,
                &query,
            )
            .await
    }
    /// Imports a specified process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups/import`.
    pub async fn import_process_group(
        &self,
        body: &crate::types::ProcessGroupUploadEntity,
    ) -> Result<crate::types::ProcessGroupEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/process-groups/import"), body)
            .await
    }
    /// Uploads a versioned flow definition and creates a process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups/upload`.
    pub async fn upload_process_group(
        &self,
    ) -> Result<crate::types::ProcessGroupEntity, NifiError> {
        let id = self.id;
        self.client
            .post_no_body(&format!("/process-groups/{id}/process-groups/upload"))
            .await
    }
}
pub struct ProcessGroupsProcessorsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsProcessorsApi<'a> {
    /// Gets all processors
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/processors`.
    ///
    /// # Parameters
    /// - `include_descendant_groups`: Whether or not to include processors from descendant process groups
    pub async fn get_processors(
        &self,
        include_descendant_groups: Option<bool>,
    ) -> Result<crate::types::ProcessorsEntity, NifiError> {
        let id = self.id;
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = include_descendant_groups {
            query.push(("includeDescendantGroups", v.to_string()));
        }
        self.client
            .get_with_query(&format!("/process-groups/{id}/processors"), &query)
            .await
    }
    /// Creates a new processor
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/processors`.
    ///
    /// # Parameters
    /// - `body`: The processor configuration details.
    pub async fn create_processor(
        &self,
        body: &crate::types::ProcessorEntity,
    ) -> Result<crate::types::ProcessorEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/processors"), body)
            .await
    }
}
pub struct ProcessGroupsRemoteProcessGroupsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsRemoteProcessGroupsApi<'a> {
    /// Gets all remote process groups
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/remote-process-groups`.
    pub async fn get_remote_process_groups(
        &self,
    ) -> Result<crate::types::RemoteProcessGroupsEntity, NifiError> {
        let id = self.id;
        self.client
            .get(&format!("/process-groups/{id}/remote-process-groups"))
            .await
    }
    /// Creates a new process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/remote-process-groups`.
    ///
    /// # Parameters
    /// - `body`: The remote process group configuration details.
    pub async fn create_remote_process_group(
        &self,
        body: &crate::types::RemoteProcessGroupEntity,
    ) -> Result<crate::types::RemoteProcessGroupEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/remote-process-groups"), body)
            .await
    }
}
pub struct ProcessGroupsReplaceRequestsApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsReplaceRequestsApi<'a> {
    /// Initiate the Replace Request of a Process Group with the given ID
    ///
    /// This will initiate the action of replacing a process group with the given process group. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a ProcessGroupReplaceRequestEntity, and the process of replacing the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /process-groups/replace-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /process-groups/replace-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/replace-requests`.
    ///
    /// # Parameters
    /// - `body`: The process group replace request entity
    pub async fn initiate_replace_process_group(
        &self,
        body: &crate::types::ProcessGroupImportEntity,
    ) -> Result<crate::types::ProcessGroupReplaceRequestEntity, NifiError> {
        let id = self.id;
        self.client
            .post(&format!("/process-groups/{id}/replace-requests"), body)
            .await
    }
}
pub struct ProcessGroupsSnippetInstanceApi<'a> {
    pub(crate) client: &'a NifiClient,
    pub(crate) id: &'a str,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> ProcessGroupsSnippetInstanceApi<'a> {
    /// Copies a snippet and discards it.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/snippet-instance`.
    ///
    /// # Parameters
    /// - `body`: The copy snippet request.
    pub async fn copy_snippet(
        &self,
        body: &crate::types::CopySnippetRequestEntity,
    ) -> Result<crate::types::FlowDto, NifiError> {
        let id = self.id;
        let e: crate::types::FlowEntity = self
            .client
            .post(&format!("/process-groups/{id}/snippet-instance"), body)
            .await?;
        Ok(e.flow)
    }
}
