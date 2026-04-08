// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The ProcessGroups API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait ProcessGroupsApi {
    /// Generates a copy response for the given copy request
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/copy`.
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
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    async fn copy(
        &self,
        id: &str,
        body: types::CopyRequestEntity,
    ) -> Result<types::CopyResponseEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "copy".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Copies a snippet and discards it.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/snippet-instance`.
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
    /// Requires `Read - /{component-type}/{uuid} - For each component in the snippet and their descendant components`.
    /// Requires `Write - if the snippet contains any restricted Processors - /restricted-components`.
    async fn copy_snippet(
        &self,
        id: &str,
        body: types::CopySnippetRequestEntity,
    ) -> Result<types::FlowDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "copy_snippet".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a connection
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/connections`.
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
    /// Requires `Write Source - /{component-type}/{uuid}`.
    /// Requires `Write Destination - /{component-type}/{uuid}`.
    async fn create_connection(
        &self,
        id: &str,
        body: types::ConnectionEntity,
    ) -> Result<types::ConnectionEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_connection".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new controller service
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/controller-services`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        body: types::ControllerServiceEntity,
    ) -> Result<types::ControllerServiceEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_controller_service_1".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a request to drop all flowfiles of all connection queues in this process group.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/empty-all-connections-requests`.
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
    /// Requires `Read - /process-groups/{uuid} - For this and all encapsulated process groups`.
    /// Requires `Write Source Data - /data/{component-type}/{uuid} - For all encapsulated connections`.
    async fn create_empty_all_connections_request(
        &self,
        id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_empty_all_connections_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a funnel
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/funnels`.
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
    async fn create_funnel(
        &self,
        id: &str,
        body: types::FunnelEntity,
    ) -> Result<types::FunnelEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_funnel".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates an input port
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/input-ports`.
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
    async fn create_input_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_input_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a label
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/labels`.
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
    async fn create_label(
        &self,
        id: &str,
        body: types::LabelEntity,
    ) -> Result<types::LabelEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_label".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates an output port
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/output-ports`.
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
    async fn create_output_port(
        &self,
        id: &str,
        body: types::PortEntity,
    ) -> Result<types::PortEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_output_port".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        parameter_context_handling_strategy: Option<types::ParameterContextHandlingStrategy>,
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new processor
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/processors`.
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
    /// Requires `Read - any referenced Controller Services - /controller-services/{uuid}`.
    /// Requires `Write - if the Processor is restricted - /restricted-components`.
    async fn create_processor(
        &self,
        id: &str,
        body: types::ProcessorEntity,
    ) -> Result<types::ProcessorEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_processor".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Creates a new process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/remote-process-groups`.
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
    async fn create_remote_process_group(
        &self,
        id: &str,
        body: types::RemoteProcessGroupEntity,
    ) -> Result<types::RemoteProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "create_remote_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
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
    /// Gets a process group for download
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/download`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        include_referenced_services: Option<bool>,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "export_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all connections
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/connections`.
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
    async fn get_connections(&self, id: &str) -> Result<types::ConnectionsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_connections".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the current status of a drop all flowfiles request.
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/empty-all-connections-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_drop_all_flowfiles_request".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all funnels
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/funnels`.
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
    async fn get_funnels(&self, id: &str) -> Result<types::FunnelsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_funnels".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all input ports
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/input-ports`.
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
    async fn get_input_ports(&self, id: &str) -> Result<types::InputPortsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_input_ports".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all labels
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/labels`.
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
    async fn get_labels(&self, id: &str) -> Result<types::LabelsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_labels".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a list of local modifications to the Process Group since it was last synchronized with the Flow Registry
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/local-modifications`.
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
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    async fn get_local_modifications(
        &self,
        id: &str,
    ) -> Result<types::FlowComparisonEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_local_modifications".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all output ports
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/output-ports`.
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
    async fn get_output_ports(&self, id: &str) -> Result<types::OutputPortsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_output_ports".to_string(),
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
    /// Gets all process groups
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/process-groups`.
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
    async fn get_process_groups(&self, id: &str) -> Result<types::ProcessGroupsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_process_groups".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all processors
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/processors`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        include_descendant_groups: Option<bool>,
    ) -> Result<types::ProcessorsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processors".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all remote process groups
    ///
    /// Calls `GET /nifi-api/process-groups/{id}/remote-process-groups`.
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
    async fn get_remote_process_groups(
        &self,
        id: &str,
    ) -> Result<types::RemoteProcessGroupsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_remote_process_groups".to_string(),
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
    /// Imports a specified process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups/import`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        body: types::ProcessGroupUploadEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "import_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Initiate the Replace Request of a Process Group with the given ID
    ///
    /// This will initiate the action of replacing a process group with the given process group. This can be a lengthy process, as it will stop any Processors and disable any Controller Services necessary to perform the action and then restart them. As a result, the endpoint will immediately return a ProcessGroupReplaceRequestEntity, and the process of replacing the flow will occur asynchronously in the background. The client may then periodically poll the status of the request by issuing a GET request to /process-groups/replace-requests/{requestId}. Once the request is completed, the client is expected to issue a DELETE request to /process-groups/replace-requests/{requestId}. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/replace-requests`.
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
    /// Requires `Write - /process-groups/{uuid}`.
    /// Requires `Read - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - /{component-type}/{uuid} - For all encapsulated components`.
    /// Requires `Write - if the snapshot contains any restricted components - /restricted-components`.
    /// Requires `Read - /parameter-contexts/{uuid} - For any Parameter Context that is referenced by a Property that is changed, added, or removed`.
    async fn initiate_replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupReplaceRequestEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "initiate_replace_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Pastes into the specified process group
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}/paste`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        body: types::PasteRequestEntity,
    ) -> Result<types::PasteResponseEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "paste".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Cancels and/or removes a request to drop all flowfiles.
    ///
    /// Calls `DELETE /nifi-api/process-groups/{id}/empty-all-connections-requests/{drop-request-id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
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
        id: &str,
        drop_request_id: &str,
    ) -> Result<types::DropRequestDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "remove_drop_request_1".to_string(),
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
    /// Replace Process Group contents with the given ID with the specified Process Group contents
    ///
    /// This endpoint is used for replication within a cluster, when replacing a flow with a new flow. It expects that the flow beingreplaced is not under version control and that the given snapshot will not modify any Processor that is currently running or any Controller Service that is enabled. Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `PUT /nifi-api/process-groups/{id}/flow-contents`.
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
    /// Requires `Write - /process-groups/{uuid}`.
    async fn replace_process_group(
        &self,
        id: &str,
        body: types::ProcessGroupImportEntity,
    ) -> Result<types::ProcessGroupImportEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "replace_process_group".to_string(),
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
        body: types::ProcessGroupEntity,
    ) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "update_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Uploads a versioned flow definition and creates a process group
    ///
    /// Calls `POST /nifi-api/process-groups/{id}/process-groups/upload`.
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
    async fn upload_process_group(&self, id: &str) -> Result<types::ProcessGroupEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "upload_process_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
