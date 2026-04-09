// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// Sub-resource trait for FlowBranchesApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowBranchesApi {
    /// Gets the branches from the specified registry for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{id}/branches`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_branches(&self) -> Result<types::FlowRegistryBranchesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_branches".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the differences between two versions of the same versioned flow, the basis of the comparison will be the first version
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/branches/{branch-id-a}/buckets/{bucket-id-a}/flows/{flow-id-a}/{version-a}/diff/branches/{branch-id-b}/buckets/{bucket-id-b}/flows/{flow-id-b}/{version-b}`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `branch_id_a`: The branch id for the base version.
    /// - `bucket_id_a`: The bucket id for the base version.
    /// - `flow_id_a`: The flow id for the base version.
    /// - `version_a`: The base version.
    /// - `branch_id_b`: The branch id for the compared version.
    /// - `bucket_id_b`: The bucket id for the compared version.
    /// - `flow_id_b`: The flow id for the compared version.
    /// - `version_b`: The compared version.
    /// - `offset`: Must be a non-negative number. Specifies the starting point of the listing. 0 means start from the beginning.
    /// - `limit`: Limits the number of differences listed. This might lead to partial result. 0 means no limitation is applied.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_version_differences(
        &self,
        registry_id: &str,
        branch_id_a: &str,
        bucket_id_a: &str,
        flow_id_a: &str,
        version_a: &str,
        branch_id_b: &str,
        bucket_id_b: &str,
        flow_id_b: &str,
        version_b: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<types::FlowComparisonEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_version_differences".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowBreadcrumbsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowBreadcrumbsApi {
    /// Gets the breadcrumbs for a process group
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/breadcrumbs`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_breadcrumbs(&self) -> Result<types::FlowBreadcrumbEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_breadcrumbs".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowBucketsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowBucketsApi {
    /// Gets the buckets from the specified registry for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{id}/buckets`.
    ///
    /// # Parameters
    /// - `branch`: The name of a branch to get the buckets from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_buckets(
        &self,
        branch: Option<&str>,
    ) -> Result<types::FlowRegistryBucketsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_buckets".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the details of a flow from the specified registry and bucket for the specified flow for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/buckets/{bucket-id}/flows/{flow-id}/details`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `bucket_id`: The bucket id.
    /// - `flow_id`: The flow id.
    /// - `branch`: The name of a branch to get the flow from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_details(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_details".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the flows from the specified registry and bucket for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/buckets/{bucket-id}/flows`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `bucket_id`: The bucket id.
    /// - `branch`: The name of a branch to get the flows from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flows(
        &self,
        registry_id: &str,
        bucket_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flows".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the flow versions from the specified registry and bucket for the specified flow for the current user
    ///
    /// Calls `GET /nifi-api/flow/registries/{registry-id}/buckets/{bucket-id}/flows/{flow-id}/versions`.
    ///
    /// # Parameters
    /// - `registry_id`: The registry client id.
    /// - `bucket_id`: The bucket id.
    /// - `flow_id`: The flow id.
    /// - `branch`: The name of a branch to get the flow versions from. If not specified the default branch of the registry client will be used.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_versions(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<types::VersionedFlowSnapshotMetadataSetEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_versions".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowBulletinsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowBulletinsApi {
    /// Clears bulletins for components in the specified Process Group.
    ///
    /// Calls `POST /nifi-api/flow/process-groups/{id}/bulletins/clear-requests`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    /// Requires `Write - /process-groups/{uuid} - For the process group`.
    /// Requires `Write - /{component-type}/{uuid} - For every component having bulletins cleared`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn clear_bulletins_1(
        &self,
        body: &types::ClearBulletinsForGroupRequestEntity,
    ) -> Result<types::ClearBulletinsForGroupResultsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "clear_bulletins_1".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowControllerServicesApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowControllerServicesApi {
    /// Enable or disable Controller Services in the specified Process Group.
    ///
    /// Calls `PUT /nifi-api/flow/process-groups/{id}/controller-services`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    /// Requires `Write - /{component-type}/{uuid} or /operation/{component-type}/{uuid} - For every service being enabled/disabled`.
    async fn activate_controller_services(
        &self,
        body: &types::ActivateControllerServicesEntity,
    ) -> Result<types::ActivateControllerServicesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "activate_controller_services".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all controller services
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/controller-services`.
    ///
    /// # Parameters
    /// - `include_ancestor_groups`: Whether or not to include parent/ancestor process groups
    /// - `include_descendant_groups`: Whether or not to include descendant process groups
    /// - `include_referencing_components`: Whether or not to include services' referencing components in the response
    /// - `ui_only`
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_controller_services_from_group(
        &self,
        include_ancestor_groups: Option<bool>,
        include_descendant_groups: Option<bool>,
        include_referencing_components: Option<bool>,
        ui_only: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_services_from_group".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowStatisticsApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowStatisticsApi {
    /// Gets statistics for a connection
    ///
    /// Calls `GET /nifi-api/flow/connections/{id}/statistics`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the statistics.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_connection_statistics(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatisticsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_connection_statistics".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// Sub-resource trait for FlowStatusApi.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowStatusApi {
    /// Gets status for a connection
    ///
    /// Calls `GET /nifi-api/flow/connections/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_connection_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ConnectionStatusEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_connection_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the status history for a connection
    ///
    /// Calls `GET /nifi-api/flow/connections/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_connection_status_history(&self) -> Result<types::StatusHistoryEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_connection_status_history".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets status for an input port
    ///
    /// Calls `GET /nifi-api/flow/input-ports/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_input_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_input_port_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets status for an output port
    ///
    /// Calls `GET /nifi-api/flow/output-ports/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_output_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::PortStatusEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_output_port_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the status for a process group
    ///
    /// The status for a process group includes status for all descendent components. When invoked on the root group with recursive set to true, it will return the current status of every component in the flow.
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/status`.
    ///
    /// # Parameters
    /// - `recursive`: Whether all descendant groups and the status of their content will be included. Optional, defaults to false
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_process_group_status(
        &self,
        recursive: Option<bool>,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessGroupStatusEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_process_group_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets status history for a remote process group
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_process_group_status_history(
        &self,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_process_group_status_history".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets status for a processor
    ///
    /// Calls `GET /nifi-api/flow/processors/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_processor_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::ProcessorStatusEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processor_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets status history for a processor
    ///
    /// Calls `GET /nifi-api/flow/processors/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_processor_status_history(&self) -> Result<types::StatusHistoryEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processor_status_history".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets status for a remote process group
    ///
    /// Calls `GET /nifi-api/flow/remote-process-groups/{id}/status`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_remote_process_group_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::RemoteProcessGroupStatusEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_remote_process_group_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the status history
    ///
    /// Calls `GET /nifi-api/flow/remote-process-groups/{id}/status/history`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_remote_process_group_status_history(
        &self,
    ) -> Result<types::StatusHistoryEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_remote_process_group_status_history".to_string(),
            version: "unknown".to_string(),
        })
    }
}
/// The Flow API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowApi {
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The registry id.
    type FlowBranchesApi<'b>: FlowBranchesApi
    where
        Self: 'b;
    fn branches<'b>(&'b self, id: &'b str) -> Self::FlowBranchesApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type FlowBreadcrumbsApi<'b>: FlowBreadcrumbsApi
    where
        Self: 'b;
    fn breadcrumbs<'b>(&'b self, id: &'b str) -> Self::FlowBreadcrumbsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The registry id.
    type FlowBucketsApi<'b>: FlowBucketsApi
    where
        Self: 'b;
    fn buckets<'b>(&'b self, id: &'b str) -> Self::FlowBucketsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type FlowBulletinsApi<'b>: FlowBulletinsApi
    where
        Self: 'b;
    fn bulletins<'b>(&'b self, id: &'b str) -> Self::FlowBulletinsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    type FlowControllerServicesApi<'b>: FlowControllerServicesApi
    where
        Self: 'b;
    fn controller_services<'b>(&'b self, id: &'b str) -> Self::FlowControllerServicesApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    type FlowStatisticsApi<'b>: FlowStatisticsApi
    where
        Self: 'b;
    fn statistics<'b>(&'b self, id: &'b str) -> Self::FlowStatisticsApi<'b>;
    /// Returns a sub-resource accessor for config operations.
    ///
    /// # Parameters
    /// - `id`: The connection id.
    type FlowStatusApi<'b>: FlowStatusApi
    where
        Self: 'b;
    fn status<'b>(&'b self, id: &'b str) -> Self::FlowStatusApi<'b>;
    /// Download a snapshot of the given reporting tasks and any controller services they use
    ///
    /// Calls `GET /nifi-api/flow/reporting-tasks/download`.
    ///
    /// # Parameters
    /// - `reporting_task_id`: Specifies a reporting task id to export. If not specified, all reporting tasks will be exported.
    ///
    /// # Errors
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn download_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "download_reporting_task_snapshot".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Generates a client id.
    ///
    /// Calls `GET /nifi-api/flow/client-id`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn generate_client_id(&self) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "generate_client_id".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves details about this NiFi to put in the About dialog
    ///
    /// Calls `GET /nifi-api/flow/about`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_about_info(&self) -> Result<types::AboutDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_about_info".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets an action
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/history/{id}`.
    ///
    /// # Parameters
    /// - `id`: The action id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_action(&self, id: &str) -> Result<types::ActionEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_action".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the additional details for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/additional-details/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The processor type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The additional details for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_additional_details(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::AdditionalDetailsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_additional_details".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns all flow analysis results currently in effect
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis/results`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_all_flow_analysis_results(
        &self,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_all_flow_analysis_results".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the banners for this NiFi
    ///
    /// Calls `GET /nifi-api/flow/banners`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_banners(&self) -> Result<types::BannerDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_banners".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets current bulletins
    ///
    /// Calls `GET /nifi-api/flow/bulletin-board`.
    ///
    /// # Parameters
    /// - `after`: Includes bulletins with an id after this value.
    /// - `source_name`: Includes bulletins originating from this sources whose name match this regular expression.
    /// - `message`: Includes bulletins whose message that match this regular expression.
    /// - `source_id`: Includes bulletins originating from this sources whose id match this regular expression.
    /// - `group_id`: Includes bulletins originating from this sources whose group id match this regular expression.
    /// - `limit`: The number of bulletins to limit the response to.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    /// Requires `Read - /{component-type}/{uuid} - For component specific bulletins`.
    async fn get_bulletin_board(
        &self,
        after: Option<&str>,
        source_name: Option<&str>,
        message: Option<&str>,
        source_id: Option<&str>,
        group_id: Option<&str>,
        limit: Option<&str>,
    ) -> Result<types::BulletinBoardDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_bulletin_board".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves Controller level bulletins
    ///
    /// Calls `GET /nifi-api/flow/controller/bulletins`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    /// Requires `Read - /controller - For controller bulletins`.
    /// Requires `Read - /controller-services/{uuid} - For controller service bulletins`.
    /// Requires `Read - /reporting-tasks/{uuid} - For reporting task bulletins`.
    async fn get_bulletins(&self) -> Result<types::ControllerBulletinsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_bulletins".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// The cluster summary for this NiFi
    ///
    /// Calls `GET /nifi-api/flow/cluster/summary`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_cluster_summary(&self) -> Result<types::ClusterSummaryDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_cluster_summary".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets configuration history for a component
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/history/components/{componentId}`.
    ///
    /// # Parameters
    /// - `component_id`: The component id.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    /// Requires `Read underlying component - /{component-type}/{uuid}`.
    async fn get_component_history(
        &self,
        component_id: &str,
    ) -> Result<types::ComponentHistoryDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_component_history".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the registered content viewers
    ///
    /// Calls `GET /nifi-api/flow/content-viewers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_content_viewers(&self) -> Result<types::ContentViewerEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_content_viewers".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the Controller Service Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/controller-service-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The controller service type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The controller service definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_controller_service_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ControllerServiceDefinition, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_service_definition".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the types of controller services that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/controller-service-types`.
    ///
    /// # Parameters
    /// - `service_type`: If specified, will only return controller services that are compatible with this type of service.
    /// - `service_bundle_group`: If serviceType specified, is the bundle group of the serviceType.
    /// - `service_bundle_artifact`: If serviceType specified, is the bundle artifact of the serviceType.
    /// - `service_bundle_version`: If serviceType specified, is the bundle version of the serviceType.
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type_filter`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_controller_service_types(
        &self,
        service_type: Option<&str>,
        service_bundle_group: Option<&str>,
        service_bundle_artifact: Option<&str>,
        service_bundle_version: Option<&str>,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        type_filter: Option<&str>,
    ) -> Result<types::ControllerServiceTypesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_service_types".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets controller services for reporting tasks
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/flow/controller/controller-services`.
    ///
    /// # Parameters
    /// - `ui_only`
    /// - `include_referencing_components`: Whether or not to include services' referencing components in the response
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_controller_services_from_controller(
        &self,
        ui_only: Option<bool>,
        include_referencing_components: Option<bool>,
    ) -> Result<types::ControllerServicesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_services_from_controller".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the current status of this NiFi
    ///
    /// Calls `GET /nifi-api/flow/status`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_controller_status(&self) -> Result<types::ControllerStatusDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_controller_status".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the user identity of the user making the request
    ///
    /// Calls `GET /nifi-api/flow/current-user`.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_current_user(&self) -> Result<types::CurrentUserEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_current_user".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets a process group
    ///
    /// If the uiOnly query parameter is provided with a value of true, the returned entity may only contain fields that are necessary for rendering the NiFi User Interface. As such, the selected fields may change at any time, even during incremental releases, without warning. As a result, this parameter should not be provided by any client other than the UI.
    ///
    /// Calls `GET /nifi-api/flow/process-groups/{id}`.
    ///
    /// # Parameters
    /// - `id`: The process group id.
    /// - `ui_only`
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flow(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<types::ProcessGroupFlowEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Returns flow analysis results produced by the analysis of a given process group
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis/results/{processGroupId}`.
    ///
    /// # Parameters
    /// - `process_group_id`: The id of the process group representing (a part of) the flow to be analyzed.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flow_analysis_results(
        &self,
        process_group_id: &str,
    ) -> Result<types::FlowAnalysisResultEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_results".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the Flow Analysis Rule Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis-rule-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The flow analysis rule type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The flow analysis rule definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flow_analysis_rule_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowAnalysisRuleDefinition, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_rule_definition".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the types of available Flow Analysis Rules
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/flow-analysis-rule-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flow_analysis_rule_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::FlowAnalysisRuleTypesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_analysis_rule_types".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the configuration for this NiFi flow
    ///
    /// Calls `GET /nifi-api/flow/config`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flow_config(&self) -> Result<types::FlowConfigurationDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_config".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all metrics for the flow from a particular node
    ///
    /// Calls `GET /nifi-api/flow/metrics/{producer}`.
    ///
    /// # Parameters
    /// - `producer`: The producer for flow file metrics. Each producer may have its own output format.
    /// - `included_registries`: Set of included metrics registries. Duplicate the parameter to include multiple registries. All registries are included by default.
    /// - `sample_name`: Regular Expression Pattern to be applied against the sample name field
    /// - `sample_label_value`: Regular Expression Pattern to be applied against the sample label value field
    /// - `root_field_name`: Name of the first field of JSON object. Applicable for JSON producer only.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_flow_metrics(
        &self,
        producer: &str,
        included_registries: Option<types::IncludedRegistries>,
        sample_name: Option<&str>,
        sample_label_value: Option<&str>,
        root_field_name: Option<&str>,
        flow_metrics_reporting_strategy: Option<types::FlowMetricsReportingStrategy>,
    ) -> Result<(), NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_metrics".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the Flow Registry Client Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/flow-registry-client-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The flow registry client type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The flow registry client definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn get_flow_registry_client_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::FlowRegistryClientDefinition, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_flow_registry_client_definition".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all listen ports configured on this NiFi that the current user has access to
    ///
    /// Calls `GET /nifi-api/flow/listen-ports`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    ///
    /// *Supported in NiFi: 2.7.2, 2.8.0*
    async fn get_listen_ports(&self) -> Result<types::ListenPortsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_listen_ports".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all Parameter Contexts
    ///
    /// Calls `GET /nifi-api/flow/parameter-contexts`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /parameter-contexts/{id} for each Parameter Context`.
    async fn get_parameter_contexts(&self) -> Result<types::ParameterContextsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_contexts".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the Parameter Provider Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/parameter-provider-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The parameter provider type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The reporting task definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_parameter_provider_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ParameterProviderDefinition, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_provider_definition".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the types of parameter providers that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/parameter-provider-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_parameter_provider_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ParameterProviderTypesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_provider_types".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all parameter providers
    ///
    /// Calls `GET /nifi-api/flow/parameter-providers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_parameter_providers(&self) -> Result<types::ParameterProvidersEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_parameter_providers".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the types of prioritizers that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/prioritizers`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_prioritizers(&self) -> Result<types::PrioritizerTypesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_prioritizers".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the Processor Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/processor-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The processor type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The processor definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_processor_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ProcessorDefinition, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processor_definition".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the types of processors that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/processor-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_processor_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ProcessorTypesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_processor_types".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the listing of available flow registry clients
    ///
    /// Calls `GET /nifi-api/flow/registries`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_registry_clients(&self) -> Result<types::FlowRegistryClientsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_registry_clients".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the Reporting Task Definition for the specified component type.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/reporting-task-definition/{group}/{artifact}/{version}/{type}`.
    ///
    /// # Parameters
    /// - `group`: The bundle group
    /// - `artifact`: The bundle artifact
    /// - `version`: The bundle version
    /// - `type`: The reporting task type
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The reporting task definition for the coordinates could not be located.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_reporting_task_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<types::ReportingTaskDefinition, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_reporting_task_definition".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Get a snapshot of the given reporting tasks and any controller services they use
    ///
    /// Calls `GET /nifi-api/flow/reporting-tasks/snapshot`.
    ///
    /// # Parameters
    /// - `reporting_task_id`: Specifies a reporting task id to export. If not specified, all reporting tasks will be exported.
    ///
    /// # Errors
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<types::VersionedReportingTaskSnapshot, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_reporting_task_snapshot".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the types of reporting tasks that this NiFi supports
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/reporting-task-types`.
    ///
    /// # Parameters
    /// - `bundle_group_filter`: If specified, will only return types that are a member of this bundle group.
    /// - `bundle_artifact_filter`: If specified, will only return types that are a member of this bundle artifact.
    /// - `type`: If specified, will only return types whose fully qualified classname matches.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_reporting_task_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<types::ReportingTaskTypesEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_reporting_task_types".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets all reporting tasks
    ///
    /// Calls `GET /nifi-api/flow/reporting-tasks`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_reporting_tasks(&self) -> Result<types::ReportingTasksEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_reporting_tasks".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Retrieves the runtime manifest for this NiFi instance.
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/runtime-manifest`.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn get_runtime_manifest(&self) -> Result<types::RuntimeManifest, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_runtime_manifest".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets configuration history
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/history`.
    ///
    /// # Parameters
    /// - `offset`: The offset into the result set.
    /// - `count`: The number of actions to return.
    /// - `sort_column`: The field to sort on.
    /// - `sort_order`: The direction to sort.
    /// - `start_date`: Include actions after this date.
    /// - `end_date`: Include actions before this date.
    /// - `user_identity`: Include actions performed by this user.
    /// - `source_id`: Include actions on this component.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn query_history(
        &self,
        offset: &str,
        count: &str,
        sort_column: Option<&str>,
        sort_order: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        user_identity: Option<&str>,
        source_id: Option<&str>,
    ) -> Result<types::HistoryDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "query_history".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Schedule or unschedule components in the specified Process Group.
    ///
    /// Calls `PUT /nifi-api/flow/process-groups/{id}`.
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
    /// Requires `Read - /flow`.
    /// Requires `Write - /{component-type}/{uuid} or /operation/{component-type}/{uuid} - For every component being scheduled/unscheduled`.
    async fn schedule_components(
        &self,
        id: &str,
        body: &types::ScheduleComponentsEntity,
    ) -> Result<types::ScheduleComponentsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "schedule_components".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Searches the cluster for a node with the specified address
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/flow/cluster/search-results`.
    ///
    /// # Parameters
    /// - `q`: Node address to search for.
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn search_cluster(
        &self,
        q: &str,
    ) -> Result<types::ClusterSearchResultsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "search_cluster".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Performs a search against this NiFi using the specified search term
    ///
    /// Only search results from authorized components will be returned.
    ///
    /// Calls `GET /nifi-api/flow/search-results`.
    ///
    /// # Parameters
    /// - `q`
    /// - `a`
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /flow`.
    async fn search_flow(
        &self,
        q: Option<&str>,
        a: Option<&str>,
    ) -> Result<types::SearchResultsDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "search_flow".to_string(),
            version: "unknown".to_string(),
        })
    }
}
