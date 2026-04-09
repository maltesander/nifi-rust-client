// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for ProcessGroupsConnectionsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsConnectionsApi {
    /// Creates a connection
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/connections`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Write Source - /{component-type}/{uuid}`.
    /// Requires `Write Destination - /{component-type}/{uuid}`.
    async fn create_connection(
        &self,
        body: &types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_connection".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all connections
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/connections`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_connections(&self) -> Result<types::ConnectionsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_connections".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsControllerServicesApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsControllerServicesApi {
    /// Creates a new controller service
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/controller-services`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    /// Requires `Write - if the Controller Service is restricted - /restricted-components`.
    async fn create_controller_service_1(
        &self,
        body: &types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_controller_service_1".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsCopyApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsCopyApi {
    /// Generates a copy response for the given copy request
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/copy`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    async fn copy(
        &self,
        body: &types::CopyRequestEntity,
    ) -> Result<types::CopyResponseEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "copy".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsDownloadApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsDownloadApi {
    /// Gets a process group for download
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/download`.
    ///
    /// # Parameters
    /// - `include_referenced_services`: If referenced services from outside the target group should be included
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn export_process_group(
        &self,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "export_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsEmptyAllConnectionsRequestsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsEmptyAllConnectionsRequestsApi {
    /// Creates a request to drop all flowfiles of all connection queues in this process group.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/empty-all-connections-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid} - For this and all encapsulated process groups`.
    /// Requires `Write Source Data - /data/{component-type}/{uuid} - For all encapsulated connections`.
    async fn create_empty_all_connections_request(
        &self,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_empty_all_connections_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the current status of a drop all flowfiles request.
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/empty-all-connections-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `drop_request_id`: The drop request id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid} - For this and all encapsulated process groups`.
    /// Requires `Write Source Data - /data/{component-type}/{uuid} - For all encapsulated connections`.
    async fn get_drop_all_flowfiles_request(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_drop_all_flowfiles_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Cancels and/or removes a request to drop all flowfiles.
    ///
    /// Calls `DELETE /nifi-api/process-groups/{id}/empty-all-connections-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `drop_request_id`: The drop request id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid} - For this and all encapsulated process groups`.
    /// Requires `Write Source Data - /data/{component-type}/{uuid} - For all encapsulated connections`.
    async fn remove_drop_request_1(
        &self,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_drop_request_1".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsFlowContentsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsFlowContentsApi {
    /// Replace Process Group contents with the given ID with the specified Process Group contents
    ///
    /// This endpoint is used for replication within a cluster, when replacing a flow with a new flow. It expects that the flow beingreplaced is not under version control and that the given snapshot will not modify any Processor that is currently running or any Controller Service that is enabled. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}/flow-contents`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Write - /process-groups/{uuid}`.
    async fn replace_process_group(
        &self,
        body: &types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupImportEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "replace_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsFunnelsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsFunnelsApi {
    /// Creates a funnel
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/funnels`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn create_funnel(
        &self,
        body: &types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_funnel".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all funnels
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/funnels`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_funnels(&self) -> Result<types::FunnelsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_funnels".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsInputPortsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsInputPortsApi {
    /// Creates an input port
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/input-ports`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn create_input_port(
        &self,
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_input_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all input ports
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/input-ports`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_input_ports(&self) -> Result<types::InputPortsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_input_ports".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsLabelsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsLabelsApi {
    /// Creates a label
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/labels`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn create_label(
        &self,
        body: &types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_label".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all labels
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/labels`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_labels(&self) -> Result<types::LabelsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_labels".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsLocalModificationsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsLocalModificationsApi {
    /// Gets a list of local modifications to the Process Group since it was last synchronized with the Flow Registry
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/local-modifications`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    async fn get_local_modifications(&self) -> Result<types::FlowComparisonEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_local_modifications".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsOutputPortsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsOutputPortsApi {
    /// Creates an output port
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/output-ports`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn create_output_port(
        &self,
        body: &types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_output_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all output ports
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/output-ports`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_output_ports(&self) -> Result<types::OutputPortsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_output_ports".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsPasteApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsPasteApi {
    /// Pastes into the specified process group
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}/paste`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn paste(
        &self,
        body: &types::PasteRequestEntity,
    ) -> Result<types::PasteResponseEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "paste".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsProcessGroupsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsProcessGroupsApi {
    /// Creates a process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups`.
    ///
    /// # Parameters
    /// - `parameter_context_handling_strategy`: Handling Strategy controls whether to keep or replace Parameter Contexts
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn create_process_group(
        &self,
        parameter_context_handling_strategy: Option<types::ParameterContextHandlingStrategy>,
        body: &types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all process groups
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/process-groups`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_process_groups(&self) -> Result<types::ProcessGroupsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_process_groups".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Imports a specified process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups/import`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn import_process_group(
        &self,
        body: &types::ProcessGroupUploadEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "import_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Uploads a versioned flow definition and creates a process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups/upload`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn upload_process_group(&self) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "upload_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsProcessorsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsProcessorsApi {
    /// Creates a new processor
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/processors`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    /// Requires `Write - if the Processor is restricted - /restricted-components`.
    async fn create_processor(
        &self,
        body: &types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_processor".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all processors
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/processors`.
    ///
    /// # Parameters
    /// - `include_descendant_groups`: Whether or not to include processors from descendant process groups
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_processors(
        &self,
        include_descendant_groups: Option<bool>,
    ) -> Result<types::ProcessorsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processors".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsRemoteProcessGroupsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsRemoteProcessGroupsApi {
    /// Creates a new process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/remote-process-groups`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn create_remote_process_group(
        &self,
        body: &types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_remote_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all remote process groups
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/remote-process-groups`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_remote_process_groups(
        &self,
    ) -> Result<types::RemoteProcessGroupsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_remote_process_groups".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsReplaceRequestsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsReplaceRequestsApi {
    /// Initiate the Replace Request of a Process Group with the given ID
    ///
    /// This will initiate the action of replacing a process group with the given process group. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a ProcessGroupReplaceRequestEntity, and the process of replacing the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /process-groups/replace-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /process-groups/replace-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/replace-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - if the snapshot contains any restricted components - /restricted-components`.
    /// Requires `Read - /parameter-contexts/{uuid} - For any Parameter Context that is referenced by a Property that is changed, added, or removed`.
    async fn initiate_replace_process_group(
        &self,
        body: &types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "initiate_replace_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for ProcessGroupsSnippetInstanceApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsSnippetInstanceApi {
    /// Copies a snippet and discards it.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/snippet-instance`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - /{component-type}/{uuid} - For each component in the snippet and their descendant components`.
    /// Requires `Write - if the snippet contains any restricted Processors - /restricted-components`.
    async fn copy_snippet(
        &self,
        body: &types::CopySnippetRequestEntity,
    ) -> Result<types::FlowDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "copy_snippet".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The ProcessGroups API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsConnectionsApi<'b>: ProcessGroupsConnectionsApi
    where
        Self: 'b;
    fn connections<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsConnectionsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsControllerServicesApi<'b>: ProcessGroupsControllerServicesApi
    where
        Self: 'b;
    fn controller_services<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsControllerServicesApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsCopyApi<'b>: ProcessGroupsCopyApi
    where
        Self: 'b;
    fn copy<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsCopyApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsDownloadApi<'b>: ProcessGroupsDownloadApi
    where
        Self: 'b;
    fn download<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsDownloadApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsEmptyAllConnectionsRequestsApi<
        'b,
    >: ProcessGroupsEmptyAllConnectionsRequestsApi
    where
        Self: 'b;
    fn empty_all_connections_requests<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsEmptyAllConnectionsRequestsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsFlowContentsApi<'b>: ProcessGroupsFlowContentsApi
    where
        Self: 'b;
    fn flow_contents<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsFlowContentsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsFunnelsApi<'b>: ProcessGroupsFunnelsApi
    where
        Self: 'b;
    fn funnels<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsFunnelsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsInputPortsApi<'b>: ProcessGroupsInputPortsApi
    where
        Self: 'b;
    fn input_ports<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsInputPortsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsLabelsApi<'b>: ProcessGroupsLabelsApi
    where
        Self: 'b;
    fn labels<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsLabelsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsLocalModificationsApi<'b>: ProcessGroupsLocalModificationsApi
    where
        Self: 'b;
    fn local_modifications<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsLocalModificationsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsOutputPortsApi<'b>: ProcessGroupsOutputPortsApi
    where
        Self: 'b;
    fn output_ports<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsOutputPortsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsPasteApi<'b>: ProcessGroupsPasteApi
    where
        Self: 'b;
    fn paste<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsPasteApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsProcessGroupsApi<'b>: ProcessGroupsProcessGroupsApi
    where
        Self: 'b;
    fn process_groups<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsProcessGroupsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsProcessorsApi<'b>: ProcessGroupsProcessorsApi
    where
        Self: 'b;
    fn processors<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsProcessorsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsRemoteProcessGroupsApi<'b>: ProcessGroupsRemoteProcessGroupsApi
    where
        Self: 'b;
    fn remote_process_groups<'b>(
        &'b self,
        id: &'b str,
    ) -> Self::ProcessGroupsRemoteProcessGroupsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsReplaceRequestsApi<'b>: ProcessGroupsReplaceRequestsApi
    where
        Self: 'b;
    fn replace_requests<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsReplaceRequestsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type ProcessGroupsSnippetInstanceApi<'b>: ProcessGroupsSnippetInstanceApi
    where
        Self: 'b;
    fn snippet_instance<'b>(&'b self, id: &'b str) -> Self::ProcessGroupsSnippetInstanceApi<'b>;
    /// Deletes the Replace Request with the given ID
    ///
    /// Deletes the Replace Request with the given ID. After a request is created via a POST to /process-groups/{id}/replace-requests, it is expected that the client will properly clean up the request by DELETE'ing it, once the Replace process has completed. If the request is deleted before the request completes, then the Replace request will finish the step that it is currently performing and then will cancel any subsequent steps. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `DELETE /nifi-api/process-groups/replace-requests/{id}`.
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
    async fn delete_replace_process_group_request(
        &self,
        id: &str,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "delete_replace_process_group_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a process group
    ///
    /// Calls `GET /nifi-api/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /process-groups/{uuid}`.
    async fn get_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns the Replace Request with the given ID
    ///
    /// Returns the Replace Request with the given ID. Once a Replace Request has been created by performing a POST to /process-groups/{id}/replace-requests, that request can subsequently be retrieved via this endpoint, and the request that is fetched will contain the updated state, such as percent complete, the current state of the request, and any failures. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/process-groups/replace-requests/{id}`.
    ///
    /// # Parameters
    /// - `id`: The ID of the Replace Request
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
    async fn get_replace_process_group_request(
        &self,
        id: &str,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_replace_process_group_request".to_string(),
            version: "unknown".to_string(),
        })
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
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Write - Parent Process Group - /process-groups/{uuid}`.
    /// Requires `Read - any referenced Controller Services by any encapsulated components - /controller-services/{uuid}`.
    /// Requires `Write - /{component-type}/{uuid} - For all encapsulated components`.
    async fn remove_process_group(
        &self,
        id: &str,
        version: Option<&str>,
        client_id: Option<&str>,
        disconnected_node_acknowledged: Option<bool>,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Updates a process group
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Write - /process-groups/{uuid}`.
    async fn update_process_group(
        &self,
        id: &str,
        body: &types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
