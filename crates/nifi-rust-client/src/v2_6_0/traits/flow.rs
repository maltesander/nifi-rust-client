// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// Sub-resource trait for the `branches` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowBranchesApi {
    /// Gets the branches from the specified registry for the current user
    async fn get_branches(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowRegistryBranchesEntity, NifiError>;
    /// Gets the differences between two versions of the same versioned flow, the basis of the comparison will be the first version
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
    ) -> Result<crate::v2_6_0::types::FlowComparisonEntity, NifiError>;
}
/// Sub-resource trait for the `breadcrumbs` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowBreadcrumbsApi {
    /// Gets the breadcrumbs for a process group
    async fn get_breadcrumbs(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowBreadcrumbEntity, NifiError>;
}
/// Sub-resource trait for the `buckets` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowBucketsApi {
    /// Gets the buckets from the specified registry for the current user
    async fn get_buckets(
        &self,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::FlowRegistryBucketsEntity, NifiError>;
    /// Gets the flows from the specified registry and bucket for the current user
    async fn get_flows(
        &self,
        registry_id: &str,
        bucket_id: &str,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowsEntity, NifiError>;
    /// Gets the details of a flow from the specified registry and bucket for the specified flow for the current user
    async fn get_details(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowDto, NifiError>;
    /// Gets the flow versions from the specified registry and bucket for the specified flow for the current user
    async fn get_versions(
        &self,
        registry_id: &str,
        bucket_id: &str,
        flow_id: &str,
        branch: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedFlowSnapshotMetadataSetEntity, NifiError>;
}
/// Sub-resource trait for the `controller_services` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowControllerServicesApi {
    /// Gets all controller services
    async fn get_controller_services_from_group(
        &self,
        include_ancestor_groups: Option<bool>,
        include_descendant_groups: Option<bool>,
        include_referencing_components: Option<bool>,
        ui_only: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ControllerServicesEntity, NifiError>;
    /// Enable or disable Controller Services in the specified Process Group.
    async fn activate_controller_services(
        &self,
        body: &crate::v2_6_0::types::ActivateControllerServicesEntity,
    ) -> Result<crate::v2_6_0::types::ActivateControllerServicesEntity, NifiError>;
}
/// Sub-resource trait for the `statistics` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowStatisticsApi {
    /// Gets statistics for a connection
    async fn get_connection_statistics(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ConnectionStatisticsEntity, NifiError>;
}
/// Sub-resource trait for the `status` sub-group.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowStatusApi {
    /// Gets status for a connection
    async fn get_connection_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ConnectionStatusEntity, NifiError>;
    /// Gets the status history for a connection
    async fn get_connection_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError>;
    /// Gets status for an input port
    async fn get_input_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::PortStatusEntity, NifiError>;
    /// Gets status for an output port
    async fn get_output_port_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::PortStatusEntity, NifiError>;
    /// Gets the status for a process group
    async fn get_process_group_status(
        &self,
        recursive: Option<bool>,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProcessGroupStatusEntity, NifiError>;
    /// Gets status history for a remote process group
    async fn get_process_group_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError>;
    /// Gets status for a processor
    async fn get_processor_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProcessorStatusEntity, NifiError>;
    /// Gets status history for a processor
    async fn get_processor_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError>;
    /// Gets status for a remote process group
    async fn get_remote_process_group_status(
        &self,
        nodewise: Option<bool>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::RemoteProcessGroupStatusEntity, NifiError>;
    /// Gets the status history
    async fn get_remote_process_group_status_history(
        &self,
    ) -> Result<crate::v2_6_0::types::StatusHistoryEntity, NifiError>;
}
/// The Flow API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait FlowApi {
    fn branches<'b>(&'b self, id: &'b str) -> impl FlowBranchesApi + 'b;
    fn breadcrumbs<'b>(&'b self, id: &'b str) -> impl FlowBreadcrumbsApi + 'b;
    fn buckets<'b>(&'b self, id: &'b str) -> impl FlowBucketsApi + 'b;
    fn controller_services<'b>(&'b self, id: &'b str) -> impl FlowControllerServicesApi + 'b;
    fn statistics<'b>(&'b self, id: &'b str) -> impl FlowStatisticsApi + 'b;
    fn status<'b>(&'b self, id: &'b str) -> impl FlowStatusApi + 'b;
    /// Retrieves details about this NiFi to put in the About dialog
    async fn get_about_info(&self) -> Result<crate::v2_6_0::types::AboutDto, NifiError>;
    /// Retrieves the additional details for the specified component type.
    async fn get_additional_details(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::AdditionalDetailsEntity, NifiError>;
    /// Retrieves the banners for this NiFi
    async fn get_banners(&self) -> Result<crate::v2_6_0::types::BannerDto, NifiError>;
    /// Gets current bulletins
    async fn get_bulletin_board(
        &self,
        after: Option<&str>,
        source_name: Option<&str>,
        message: Option<&str>,
        source_id: Option<&str>,
        group_id: Option<&str>,
        limit: Option<&str>,
    ) -> Result<crate::v2_6_0::types::BulletinBoardDto, NifiError>;
    /// Generates a client id.
    async fn generate_client_id(&self) -> Result<(), NifiError>;
    /// Searches the cluster for a node with the specified address
    async fn search_cluster(
        &self,
        q: &str,
    ) -> Result<crate::v2_6_0::types::ClusterSearchResultsEntity, NifiError>;
    /// The cluster summary for this NiFi
    async fn get_cluster_summary(
        &self,
    ) -> Result<crate::v2_6_0::types::ClusterSummaryDto, NifiError>;
    /// Retrieves the configuration for this NiFi flow
    async fn get_flow_config(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowConfigurationDto, NifiError>;
    /// Retrieves the registered content viewers
    async fn get_content_viewers(
        &self,
    ) -> Result<crate::v2_6_0::types::ContentViewerEntity, NifiError>;
    /// Retrieves the Controller Service Definition for the specified component type.
    async fn get_controller_service_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ControllerServiceDefinition, NifiError>;
    /// Retrieves the types of controller services that this NiFi supports
    async fn get_controller_service_types(
        &self,
        service_type: Option<&str>,
        service_bundle_group: Option<&str>,
        service_bundle_artifact: Option<&str>,
        service_bundle_version: Option<&str>,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        type_filter: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ControllerServiceTypesEntity, NifiError>;
    /// Retrieves Controller level bulletins
    async fn get_bulletins(
        &self,
    ) -> Result<crate::v2_6_0::types::ControllerBulletinsEntity, NifiError>;
    /// Gets controller services for reporting tasks
    async fn get_controller_services_from_controller(
        &self,
        ui_only: Option<bool>,
        include_referencing_components: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ControllerServicesEntity, NifiError>;
    /// Retrieves the user identity of the user making the request
    async fn get_current_user(&self) -> Result<crate::v2_6_0::types::CurrentUserEntity, NifiError>;
    /// Retrieves the Flow Analysis Rule Definition for the specified component type.
    async fn get_flow_analysis_rule_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisRuleDefinition, NifiError>;
    /// Retrieves the types of available Flow Analysis Rules
    async fn get_flow_analysis_rule_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisRuleTypesEntity, NifiError>;
    /// Returns all flow analysis results currently in effect
    async fn get_all_flow_analysis_results(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisResultEntity, NifiError>;
    /// Returns flow analysis results produced by the analysis of a given process group
    async fn get_flow_analysis_results(
        &self,
        process_group_id: &str,
    ) -> Result<crate::v2_6_0::types::FlowAnalysisResultEntity, NifiError>;
    /// Gets configuration history
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
    ) -> Result<crate::v2_6_0::types::HistoryDto, NifiError>;
    /// Gets configuration history for a component
    async fn get_component_history(
        &self,
        component_id: &str,
    ) -> Result<crate::v2_6_0::types::ComponentHistoryDto, NifiError>;
    /// Gets an action
    async fn get_action(&self, id: &str) -> Result<crate::v2_6_0::types::ActionEntity, NifiError>;
    /// Gets all metrics for the flow from a particular node
    async fn get_flow_metrics(
        &self,
        producer: &str,
        included_registries: Option<crate::v2_6_0::types::IncludedRegistries>,
        sample_name: Option<&str>,
        sample_label_value: Option<&str>,
        root_field_name: Option<&str>,
    ) -> Result<(), NifiError>;
    /// Gets all Parameter Contexts
    async fn get_parameter_contexts(
        &self,
    ) -> Result<crate::v2_6_0::types::ParameterContextsEntity, NifiError>;
    /// Retrieves the Parameter Provider Definition for the specified component type.
    async fn get_parameter_provider_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ParameterProviderDefinition, NifiError>;
    /// Retrieves the types of parameter providers that this NiFi supports
    async fn get_parameter_provider_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ParameterProviderTypesEntity, NifiError>;
    /// Gets all parameter providers
    async fn get_parameter_providers(
        &self,
    ) -> Result<crate::v2_6_0::types::ParameterProvidersEntity, NifiError>;
    /// Retrieves the types of prioritizers that this NiFi supports
    async fn get_prioritizers(
        &self,
    ) -> Result<crate::v2_6_0::types::PrioritizerTypesEntity, NifiError>;
    /// Gets a process group
    async fn get_flow(
        &self,
        id: &str,
        ui_only: Option<bool>,
    ) -> Result<crate::v2_6_0::types::ProcessGroupFlowEntity, NifiError>;
    /// Schedule or unschedule components in the specified Process Group.
    async fn schedule_components(
        &self,
        id: &str,
        body: &crate::v2_6_0::types::ScheduleComponentsEntity,
    ) -> Result<crate::v2_6_0::types::ScheduleComponentsEntity, NifiError>;
    /// Retrieves the Processor Definition for the specified component type.
    async fn get_processor_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ProcessorDefinition, NifiError>;
    /// Retrieves the types of processors that this NiFi supports
    async fn get_processor_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ProcessorTypesEntity, NifiError>;
    /// Gets the listing of available flow registry clients
    async fn get_registry_clients(
        &self,
    ) -> Result<crate::v2_6_0::types::FlowRegistryClientsEntity, NifiError>;
    /// Retrieves the Reporting Task Definition for the specified component type.
    async fn get_reporting_task_definition(
        &self,
        group: &str,
        artifact: &str,
        version: &str,
        r#type: &str,
    ) -> Result<crate::v2_6_0::types::ReportingTaskDefinition, NifiError>;
    /// Retrieves the types of reporting tasks that this NiFi supports
    async fn get_reporting_task_types(
        &self,
        bundle_group_filter: Option<&str>,
        bundle_artifact_filter: Option<&str>,
        r#type: Option<&str>,
    ) -> Result<crate::v2_6_0::types::ReportingTaskTypesEntity, NifiError>;
    /// Gets all reporting tasks
    async fn get_reporting_tasks(
        &self,
    ) -> Result<crate::v2_6_0::types::ReportingTasksEntity, NifiError>;
    /// Download a snapshot of the given reporting tasks and any controller services they use
    async fn download_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<(), NifiError>;
    /// Get a snapshot of the given reporting tasks and any controller services they use
    async fn get_reporting_task_snapshot(
        &self,
        reporting_task_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::VersionedReportingTaskSnapshot, NifiError>;
    /// Retrieves the runtime manifest for this NiFi instance.
    async fn get_runtime_manifest(
        &self,
    ) -> Result<crate::v2_6_0::types::RuntimeManifest, NifiError>;
    /// Performs a search against this NiFi using the specified search term
    async fn search_flow(
        &self,
        q: Option<&str>,
        a: Option<&str>,
    ) -> Result<crate::v2_6_0::types::SearchResultsDto, NifiError>;
    /// Gets the current status of this NiFi
    async fn get_controller_status(
        &self,
    ) -> Result<crate::v2_6_0::types::ControllerStatusDto, NifiError>;
}
